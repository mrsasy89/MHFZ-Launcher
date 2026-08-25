use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use log::{info, debug, error, warn};
use mhf_iel::MhfConfig;

#[derive(Debug)]
pub struct MhfConfigLinux {
    pub game_folder: PathBuf,
    pub config: MhfConfig,
}

fn log_to_file(msg: &str) {
    let log_path = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()) + "/mhfz-launcher.log";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "[{}] {}", timestamp, msg);
    }
}

/// Rileva se siamo su SteamOS
fn is_steamos() -> bool {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        if content.contains("ID=steamos") || content.contains("ID=\"steamos\"") {
            return true;
        }
    }
    Command::new("which")
        .arg("steamos-readonly")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Rileva se l'hardware e' uno Steam Deck (LCD = Jupiter, OLED = Galileo)
fn is_steam_deck() -> bool {
    if let Ok(product) = std::fs::read_to_string("/sys/class/dmi/id/product_name") {
        let p = product.to_lowercase();
        if p.contains("jupiter") || p.contains("galileo") || p.contains("steam deck") {
            return true;
        }
    }
    if let Ok(board) = std::fs::read_to_string("/sys/class/dmi/id/board_vendor") {
        if board.to_lowercase().contains("valve") {
            return true;
        }
    }
    false
}

/// Modalita' grafica del gioco
#[derive(Debug, Clone, Copy, PartialEq)]
enum GraphicsMode {
    Sd, // OpenGL, leggero, compatibile con Wine Flatpak sandbox
    Hd, // DXVK/D3D9, richiede accesso a Vulkan (funziona meglio con Proton)
}

/// Determina la modalita' grafica di default in base alla piattaforma,
/// con possibilita' di override esplicito da parte dell'utente/frontend
/// tramite la variabile d'ambiente MHFZ_GRAPHICS_MODE=hd|sd
fn detect_graphics_mode() -> GraphicsMode {
    if let Ok(mode) = env::var("MHFZ_GRAPHICS_MODE") {
        match mode.to_lowercase().as_str() {
            "hd" => {
                log_to_file("🎨 Graphics mode: HD (override utente)");
                return GraphicsMode::Hd;
            }
            "sd" => {
                log_to_file("🎨 Graphics mode: SD (override utente)");
                return GraphicsMode::Sd;
            }
            _ => {}
        }
    }

    if is_steam_deck() {
        log_to_file("🎨 Graphics mode: SD (default Steam Deck)");
        GraphicsMode::Sd
    } else {
        log_to_file("🎨 Graphics mode: HD (default Steam Machine/PC)");
        GraphicsMode::Hd
    }
}

fn find_proton_experimental() -> Option<PathBuf> {
    let home = env::var("HOME").unwrap_or_else(|_| "/home/deck".to_string());

    let steam_roots = vec![
        format!("{}/.steam/steam", home),
        format!("{}/.local/share/Steam", home),
        "/run/media/mmcblk0p1/.steam/steam".to_string(),
        "/run/media/mmcblk0p1/SteamLibrary".to_string(),
    ];

    for steam_root in &steam_roots {
        let proton_path = PathBuf::from(steam_root)
            .join("steamapps/common/Proton Experimental/proton");
        if proton_path.exists() {
            log_to_file(&format!("✅ Proton Experimental found: {:?}", proton_path));
            return Some(proton_path);
        }
    }

    for steam_root in &steam_roots {
        let vdf_path = PathBuf::from(steam_root).join("steamapps/libraryfolders.vdf");
        if let Ok(content) = std::fs::read_to_string(&vdf_path) {
            for line in content.lines() {
                if line.contains("\"path\"") {
                    let path_str = line
                        .split('"')
                        .nth(3)
                        .unwrap_or("")
                        .replace("\\\\", "/");
                    let proton_path = PathBuf::from(&path_str)
                        .join("steamapps/common/Proton Experimental/proton");
                    if proton_path.exists() {
                        log_to_file(&format!("✅ Proton Experimental found in library: {:?}", proton_path));
                        return Some(proton_path);
                    }
                }
            }
        }
    }

    None
}

fn find_steam_root() -> String {
    let home = env::var("HOME").unwrap_or_else(|_| "/home/deck".to_string());
    let candidates = vec![
        format!("{}/.steam/steam", home),
        format!("{}/.local/share/Steam", home),
    ];
    for path in candidates {
        if std::path::Path::new(&path).exists() {
            return path;
        }
    }
    format!("{}/.steam/steam", home)
}

#[derive(Debug, PartialEq)]
enum WineRuntime {
    ProtonExperimental(PathBuf),
    WineFlatpak,
    WineSystem,
}

fn detect_wine_runtime(mode: GraphicsMode) -> WineRuntime {
    if mode == GraphicsMode::Hd {
        if let Some(proton_path) = find_proton_experimental() {
            log_to_file("🟣 Runtime selezionato: Proton Experimental (HD mode)");
            return WineRuntime::ProtonExperimental(proton_path);
        }
        log_to_file("⚠️ HD mode richiesto ma Proton Experimental non trovato, fallback a Wine");
    }

    if is_steamos() {
        let flatpak_has_wine = Command::new("flatpak")
            .args(["list", "--app"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("org.winehq.Wine"))
            .unwrap_or(false);

        if flatpak_has_wine {
            log_to_file("🍷 Runtime selezionato: Wine Flatpak");
            return WineRuntime::WineFlatpak;
        }
    }

    log_to_file("🍷 Runtime selezionato: Wine di sistema");
    WineRuntime::WineSystem
}

fn configure_flatpak_permissions(_game_folder: &std::path::Path) {
    log_to_file("🔐 Configuring Flatpak permissions (--filesystem=home)...");

    let output = Command::new("flatpak")
        .arg("override")
        .arg("--user")
        .arg("--filesystem=home")
        .arg("org.winehq.Wine")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            log_to_file("✅ Flatpak permissions granted (home filesystem)");
        }
        Ok(out) => {
            log_to_file(&format!("⚠️ flatpak override returned: {}", out.status));
            log_to_file(&format!("   stderr: {}", String::from_utf8_lossy(&out.stderr)));
        }
        Err(e) => {
            log_to_file(&format!("❌ Failed to configure Flatpak: {}", e));
        }
    }
}

fn install_japanese_fonts(game_folder: &std::path::Path, wineprefix: &str) {
    let mut fonts_source = game_folder.join("Font");
    if !fonts_source.exists() {
        fonts_source = game_folder.join("fonts");
    }

    if !fonts_source.exists() {
        log_to_file("⚠️ Font/ or fonts/ folder not found, skipping font installation");
        warn!("Font/ or fonts/ folder not found in game directory, skipping font installation");
        return;
    }

    let fonts_dest = std::path::Path::new(wineprefix).join("drive_c/windows/Fonts");

    if !fonts_dest.exists() {
        if let Err(e) = std::fs::create_dir_all(&fonts_dest) {
            log_to_file(&format!("❌ Failed to create Fonts directory: {}", e));
            error!("Failed to create Fonts directory: {}", e);
            return;
        }
    }

    log_to_file("🧹 Cleaning existing fonts...");
    if let Ok(entries) = std::fs::read_dir(&fonts_dest) {
        let mut removed = 0;
        for entry in entries.flatten() {
            if std::fs::remove_file(entry.path()).is_ok() {
                removed += 1;
            }
        }
        log_to_file(&format!("   Removed {} old font file(s)", removed));
    }

    log_to_file("🔤 Installing MS Gothic fonts (MAX 2 files)...");
    info!("Installing MS Gothic fonts...");

    let mut count = 0;
    let mut font_names = Vec::new();
    let allowed_fonts = ["msgothic.ttc", "MS Gothic.ttf", "msgothic.ttf"];

    if let Ok(entries) = std::fs::read_dir(&fonts_source) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_string_lossy().to_lowercase();
                let is_allowed = allowed_fonts.iter().any(|&a| filename_str == a.to_lowercase());

                if is_allowed {
                    let dest = fonts_dest.join(filename);
                    if std::fs::copy(&path, &dest).is_ok() {
                        log_to_file(&format!("   ✅ Installed: {:?}", filename));
                        font_names.push(filename.to_string_lossy().to_string());
                        count += 1;
                    }
                    if count >= 2 {
                        break;
                    }
                }
            }
        }
    }

    if count == 0 {
        log_to_file("❌ No MS Gothic fonts found!");
        error!("MS Gothic fonts not found in Font/ folder");
    } else {
        log_to_file(&format!("✅ MS Gothic fonts installed ({} file(s))", count));
        register_fonts_in_wine(wineprefix, &font_names);
    }
}

fn register_fonts_in_wine(wineprefix: &str, font_files: &[String]) {
    let mode = detect_graphics_mode();
    let runtime = detect_wine_runtime(mode);

    for font_file in font_files {
        let font_name = if font_file.to_lowercase().contains("gothic") {
            "MS Gothic & MS PGothic & MS UI Gothic (TrueType)"
        } else {
            continue;
        };

        let status = match &runtime {
            WineRuntime::ProtonExperimental(proton_path) => {
                let steam_root = find_steam_root();
                Command::new("python3")
                    .arg(proton_path)
                    .arg("run").arg("reg").arg("add")
                    .arg("HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
                    .arg("/v").arg(font_name).arg("/t").arg("REG_SZ").arg("/d").arg(font_file).arg("/f")
                    .env("STEAM_COMPAT_DATA_PATH", wineprefix)
                    .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &steam_root)
                    .env("WINEDEBUG", "-all")
                    .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
                    .status()
            }
            WineRuntime::WineFlatpak => {
                Command::new("flatpak")
                    .arg("run")
                    .arg(format!("--env=WINEPREFIX={}", wineprefix))
                    .arg("--command=wine")
                    .arg("org.winehq.Wine")
                    .arg("reg").arg("add")
                    .arg("HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
                    .arg("/v").arg(font_name).arg("/t").arg("REG_SZ").arg("/d").arg(font_file).arg("/f")
                    .env("WINEDEBUG", "-all")
                    .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
                    .status()
            }
            WineRuntime::WineSystem => {
                Command::new("wine")
                    .arg("reg").arg("add")
                    .arg("HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
                    .arg("/v").arg(font_name).arg("/t").arg("REG_SZ").arg("/d").arg(font_file).arg("/f")
                    .env("WINEPREFIX", wineprefix)
                    .env("WINEDEBUG", "-all")
                    .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
                    .status()
            }
        };

        if let Ok(s) = status {
            log_to_file(&format!("   Registry result: {}", s));
        }
    }
}

pub fn run_linux(cfg: MhfConfigLinux) -> std::io::Result<()> {
    log_to_file("════════════════════════════════════════════════════");
    log_to_file("🎮 Monster Hunter Frontier Z - Linux Launcher");
    log_to_file("════════════════════════════════════════════════════");

    info!("=== Monster Hunter Frontier - Linux Launcher ===");
    log_to_file(&format!("📁 Game folder: {:?}", cfg.game_folder));

    let config_path = cfg.game_folder.join("config.json");

    let notices_json: Vec<serde_json::Value> = cfg.config.notices.iter().map(|n| {
        serde_json::json!({ "flags": n.flags, "data": &n.data })
    }).collect();

    let friends_json: Vec<serde_json::Value> = cfg.config.friends.iter().map(|f| {
        serde_json::json!({ "cid": f.cid, "id": f.id, "name": &f.name })
    }).collect();

    let mez_stalls_str: Vec<String> = cfg.config.mez_stalls.iter().map(|s| format!("{:?}", s)).collect();

    let config_json = serde_json::json!({
        "char_id": cfg.config.char_id,
        "char_name": &cfg.config.char_name,
        "char_new": cfg.config.char_new,
        "char_hr": cfg.config.char_hr,
        "char_gr": cfg.config.char_gr,
        "char_ids": &cfg.config.char_ids,
        "user_rights": cfg.config.user_rights,
        "user_token": &cfg.config.user_token,
        "user_token_id": cfg.config.user_token_id,
        "user_name": &cfg.config.user_name,
        "user_password": &cfg.config.user_password,
        "server_host": &cfg.config.server_host,
        "server_port": cfg.config.server_port,
        "notices": notices_json,
        "version": format!("{:?}", cfg.config.version),
        "entrance_count": cfg.config.entrance_count,
        "current_ts": cfg.config.current_ts,
        "expiry_ts": cfg.config.expiry_ts,
        "messages": Vec::<String>::new(),
        "mez_event_id": cfg.config.mez_event_id,
        "mez_start": cfg.config.mez_start,
        "mez_end": cfg.config.mez_end,
        "mez_solo_tickets": cfg.config.mez_solo_tickets,
        "mez_group_tickets": cfg.config.mez_group_tickets,
        "mez_stalls": mez_stalls_str,
        "friends": friends_json,
    });

    std::fs::write(&config_path, serde_json::to_string_pretty(&config_json).unwrap())
        .map_err(|e| {
            let err_msg = format!("Failed to write config.json: {}", e);
            error!("❌ {}", err_msg);
            log_to_file(&format!("❌ {}", err_msg));
            std::io::Error::new(std::io::ErrorKind::Other, err_msg)
        })?;

    log_to_file(&format!("✅ config.json written to: {:?}", config_path));

    log_to_file("🎮 Setting up XInputPlus...");
    if let Err(e) = crate::xinput::setup_xinputplus(&cfg.game_folder) {
        log_to_file(&format!("⚠️ XInputPlus setup failed: {}", e));
        warn!("XInputPlus setup failed, controller may not work properly: {}", e);
    }

    let mode = detect_graphics_mode();
    let runtime = detect_wine_runtime(mode);
    log_to_file(&format!("🔧 Graphics mode: {:?} | Runtime: {:?}", mode, runtime));

    let mut mhf_iel_exe = cfg.game_folder.join("mhf-iel.exe");
    let mut exe_name = "mhf-iel.exe";

    if !mhf_iel_exe.exists() {
        mhf_iel_exe = cfg.game_folder.join("mhf-iel-cli.exe");
        exe_name = "mhf-iel-cli.exe";
    }

    if !mhf_iel_exe.exists() {
        let err_msg = "mhf-iel.exe or mhf-iel-cli.exe not found in game folder";
        error!("{}", err_msg);
        log_to_file(&format!("❌ {}", err_msg));
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg));
    }

    log_to_file(&format!("✅ Found game executable: {}", exe_name));

    let fontconfig_path = env::var("FONTCONFIG_PATH").unwrap_or_else(|_| "/etc/fonts".to_string());
    let fontconfig_file = env::var("FONTCONFIG_FILE").unwrap_or_else(|_| "/etc/fonts/fonts.conf".to_string());
    let xdg_data_dirs = env::var("XDG_DATA_DIRS").unwrap_or_else(|_| "/usr/share:/usr/local/share".to_string());

    let (wineprefix, compat_data_path) = match &runtime {
        WineRuntime::ProtonExperimental(_) => {
            let compat = cfg.game_folder.join("proton_pfx");
            let wine_pfx = compat.join("pfx");
            (wine_pfx.to_string_lossy().to_string(), Some(compat.to_string_lossy().to_string()))
        }
        _ => {
            let pfx = env::var("WINEPREFIX").unwrap_or_else(|_| {
                cfg.game_folder.join("pfx").to_string_lossy().to_string()
            });
            (pfx, None)
        }
    };

    log_to_file(&format!("🍷 WINEPREFIX: {}", wineprefix));

    let prefix_path = std::path::Path::new(&wineprefix);
    let system_reg = prefix_path.join("system.reg");
    let dosdevices_c = prefix_path.join("dosdevices/c:");
    let drive_c_windows = prefix_path.join("drive_c/windows");
    let need_init = !prefix_path.exists()
        || !system_reg.exists()
        || !dosdevices_c.exists()
        || !drive_c_windows.exists();

    if need_init {
        log_to_file("🔧 Prefix mancante o incompleto - inizializzazione...");
        let _ = std::fs::create_dir_all(&wineprefix);

        let init_output = match &runtime {
            WineRuntime::ProtonExperimental(proton_path) => {
                let cdp = compat_data_path.as_deref().unwrap_or("");
                let steam_root = find_steam_root();
                Command::new("python3")
                    .arg(proton_path).arg("run").arg("wineboot").arg("--init")
                    .env("STEAM_COMPAT_DATA_PATH", cdp)
                    .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &steam_root)
                    .env("WINEDEBUG", "-all")
                    .env("WINEDLLOVERRIDES", "winemenubuilder.exe=d")
                    .env("FONTCONFIG_PATH", &fontconfig_path)
                    .env("FONTCONFIG_FILE", &fontconfig_file)
                    .stdin(Stdio::null())
                    .output()
            }
            WineRuntime::WineFlatpak => {
                configure_flatpak_permissions(&cfg.game_folder);
                Command::new("flatpak")
                    .arg("run")
                    .arg(format!("--env=WINEPREFIX={}", &wineprefix))
                    .arg("--env=WINEDEBUG=-all")
                    .arg("--env=WINEDLLOVERRIDES=winemenubuilder.exe=d")
                    .arg("--command=wineboot")
                    .arg("org.winehq.Wine")
                    .arg("--init")
                    .env("FONTCONFIG_PATH", &fontconfig_path)
                    .env("FONTCONFIG_FILE", &fontconfig_file)
                    .stdin(Stdio::null())
                    .output()
            }
            WineRuntime::WineSystem => {
                Command::new("wineboot")
                    .arg("--init")
                    .env("WINEPREFIX", &wineprefix)
                    .env("WINEDEBUG", "-all")
                    .env("WINEDLLOVERRIDES", "winemenubuilder.exe=d")
                    .env("FONTCONFIG_PATH", &fontconfig_path)
                    .env("FONTCONFIG_FILE", &fontconfig_file)
                    .stdin(Stdio::null())
                    .output()
            }
        };

        match init_output {
            Ok(out) if out.status.success() => log_to_file("✅ Prefix initialized successfully"),
            Ok(out) => {
                log_to_file(&format!("⚠️ wineboot exited: {}", out.status));
                log_to_file(&format!("   stderr: {}", String::from_utf8_lossy(&out.stderr)));
            }
            Err(e) => log_to_file(&format!("❌ Failed to run wineboot: {}", e)),
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
        install_japanese_fonts(&cfg.game_folder, &wineprefix);
    } else {
        log_to_file("✅ Prefix already exists and configured");
    }

    let xauthority = env::var("XAUTHORITY").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.Xauthority", home)
    });

    log_to_file("🚀 Launching game...");
    log_to_file(&format!("   Executable: {:?}", mhf_iel_exe));
    log_to_file(&format!("   Working dir: {:?}", cfg.game_folder));

    let dll_overrides = match mode {
        GraphicsMode::Hd => "xinput1_3=n,b;dinput=n,b;dinput8=n,b;winemenubuilder.exe=d;d3d9=n,b;d3d11=n,b;dxgi=n,b",
        GraphicsMode::Sd => "xinput1_3=n,b;dinput=n,b;dinput8=n,b;winemenubuilder.exe=d",
    };

    let result = match &runtime {
        WineRuntime::ProtonExperimental(proton_path) => {
            let cdp = compat_data_path.as_deref().unwrap_or("");
            let steam_root = find_steam_root();
            let mut cmd = Command::new("setsid");
            cmd.arg("python3").arg(proton_path).arg("run").arg(&mhf_iel_exe)
                .env("STEAM_COMPAT_DATA_PATH", cdp)
                .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &steam_root)
                .env("WINEDEBUG", "-all")
                .env("WINEDLLOVERRIDES", dll_overrides)
                .env("FONTCONFIG_PATH", &fontconfig_path)
                .env("FONTCONFIG_FILE", &fontconfig_file)
                .env("XDG_DATA_DIRS", &xdg_data_dirs)
                .env("XAUTHORITY", &xauthority);
            if mode == GraphicsMode::Hd {
                cmd.env("DXVK_HUD", "fps");
            }
            cmd.current_dir(&cfg.game_folder)
                .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
                .spawn()
        }
        WineRuntime::WineFlatpak => {
            Command::new("setsid")
                .arg("flatpak").arg("run")
                .arg("--filesystem=home")
                .arg(format!("--env=WINEPREFIX={}", &wineprefix))
                .arg("org.winehq.Wine")
                .arg(&mhf_iel_exe)
                .env("WINEDEBUG", "-all")
                .env("WINEDLLOVERRIDES", dll_overrides)
                .env("FONTCONFIG_PATH", &fontconfig_path)
                .env("FONTCONFIG_FILE", &fontconfig_file)
                .env("XDG_DATA_DIRS", &xdg_data_dirs)
                .env("XAUTHORITY", &xauthority)
                .current_dir(&cfg.game_folder)
                .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
                .spawn()
        }
        WineRuntime::WineSystem => {
            Command::new("setsid")
                .arg("wine").arg(&mhf_iel_exe)
                .env("WINEPREFIX", &wineprefix)
                .env("WINEDEBUG", "-all")
                .env("WINEDLLOVERRIDES", dll_overrides)
                .env("FONTCONFIG_PATH", &fontconfig_path)
                .env("FONTCONFIG_FILE", &fontconfig_file)
                .env("XDG_DATA_DIRS", &xdg_data_dirs)
                .env("XAUTHORITY", &xauthority)
                .current_dir(&cfg.game_folder)
                .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
                .spawn()
        }
    };

    match result {
        Ok(child) => {
            log_to_file(&format!("✅ Game launched successfully (PID: {})", child.id()));
            log_to_file("════════════════════════════════════════════════════");
            info!("✅ Game launched successfully (PID: {})", child.id());
            Ok(())
        }
        Err(e) => {
            log_to_file(&format!("❌ Failed to launch game: {}", e));
            error!("❌ Failed to launch game: {}", e);
            Err(e)
        }
    }
}
