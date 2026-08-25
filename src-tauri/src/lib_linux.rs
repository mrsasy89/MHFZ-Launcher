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
            log_to_file("🎮 Detected SteamOS");
            return true;
        }
    }
    if Command::new("which")
        .arg("steamos-readonly")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        log_to_file("🎮 Detected SteamOS (via steamos-readonly)");
        return true;
    }
    log_to_file("🐧 Detected standard Linux");
    false
}

/// Trova il path di Proton Experimental (cerca in tutti i path Steam comuni)
fn find_proton_experimental() -> Option<PathBuf> {
    let home = env::var("HOME").unwrap_or_else(|_| "/home/deck".to_string());

    // Path dove Steam cerca Proton (in ordine di priorità)
    let steam_roots = vec![
        format!("{}/.steam/steam", home),
        format!("{}/.local/share/Steam", home),
        // SD card su Steam Deck
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

    // Cerca anche nelle Steam Libraries aggiuntive (libraryfolders.vdf)
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

    log_to_file("⚠️ Proton Experimental NOT found");
    None
}

/// Trova il path di Steam (per STEAM_COMPAT_CLIENT_INSTALL_PATH)
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

/// Enum che descrive il runtime Wine scelto
#[derive(Debug, PartialEq)]
enum WineRuntime {
    ProtonExperimental(PathBuf), // path al binario "proton"
    WineFlatpak,
    WineSystem,
}

/// Seleziona il runtime migliore disponibile
fn detect_wine_runtime() -> WineRuntime {
    // 1. Proton Experimental (priorità massima - migliore compatibilità)
    if let Some(proton_path) = find_proton_experimental() {
        log_to_file("🟣 Runtime selezionato: Proton Experimental");
        return WineRuntime::ProtonExperimental(proton_path);
    }

    // 2. Wine Flatpak (fallback SteamOS)
    if is_steamos() {
        let flatpak_has_wine = Command::new("flatpak")
            .args(["list", "--app"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("org.winehq.Wine"))
            .unwrap_or(false);

        if flatpak_has_wine {
            log_to_file("🍷 Runtime selezionato: Wine Flatpak (Proton non trovato)");
            return WineRuntime::WineFlatpak;
        }
    }

    // 3. Wine di sistema (fallback finale)
    log_to_file("🍷 Runtime selezionato: Wine di sistema");
    WineRuntime::WineSystem
}

/// Configura permessi Flatpak per Wine (solo se usiamo WineFlatpak)
fn configure_flatpak_permissions(game_folder: &std::path::Path) {
    log_to_file("🔐 Configuring Flatpak permissions...");
    let game_path = game_folder.to_string_lossy();

    let output = Command::new("flatpak")
        .arg("override")
        .arg("--user")
        .arg(format!("--filesystem={}", game_path))
        .arg("org.winehq.Wine")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            log_to_file(&format!("✅ Flatpak permissions granted for: {}", game_path));
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

/// ✅ Installa SOLO MS Gothic (max 2 font) e PULISCE la directory prima
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

    let fonts_dest = std::path::Path::new(wineprefix)
        .join("drive_c/windows/Fonts");

    if !fonts_dest.exists() {
        log_to_file("🔧 Creating Fonts directory...");
        if let Err(e) = std::fs::create_dir_all(&fonts_dest) {
            log_to_file(&format!("❌ Failed to create Fonts directory: {}", e));
            error!("Failed to create Fonts directory: {}", e);
            return;
        }
        log_to_file(&format!("✅ Created: {:?}", fonts_dest));
    }

    log_to_file("🧹 Cleaning existing fonts (CRITICAL for SteamOS)...");
    if let Ok(entries) = std::fs::read_dir(&fonts_dest) {
        let mut removed = 0;
        for entry in entries.flatten() {
            if let Ok(_) = std::fs::remove_file(entry.path()) {
                removed += 1;
            }
        }
        log_to_file(&format!("   Removed {} old font file(s)", removed));
    }

    log_to_file("🔤 Installing MS Gothic fonts (MAX 2 files)...");
    info!("Installing MS Gothic fonts...");

    let mut count = 0;
    let mut font_names = Vec::new();

    let allowed_fonts = [
        "msgothic.ttc",
        "MS Gothic.ttf",
        "msgothic.ttf",
    ];

    if let Ok(entries) = std::fs::read_dir(&fonts_source) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_string_lossy().to_lowercase();

                let is_allowed = allowed_fonts.iter().any(|&allowed| {
                    filename_str == allowed.to_lowercase()
                });

                if is_allowed {
                    let dest = fonts_dest.join(filename);
                    match std::fs::copy(&path, &dest) {
                        Ok(_) => {
                            log_to_file(&format!("   ✅ Installed: {:?}", filename));
                            font_names.push(filename.to_string_lossy().to_string());
                            count += 1;
                        }
                        Err(e) => {
                            log_to_file(&format!("   ❌ Failed to copy {:?}: {}", filename, e));
                        }
                    }

                    if count >= 2 {
                        log_to_file("   ⚠️ Reached max 2 fonts, stopping");
                        break;
                    }
                }
            }
        }
    }

    if count == 0 {
        log_to_file("❌ No MS Gothic fonts found! Game may not display Japanese correctly.");
        error!("MS Gothic fonts not found in Font/ folder");
    } else {
        log_to_file(&format!("✅ MS Gothic fonts installed ({} file(s))", count));
        info!("MS Gothic fonts installation complete ({} files)", count);
        log_to_file("📝 Registering fonts in Wine registry...");
        register_fonts_in_wine(wineprefix, &font_names);
    }
}

/// Registra i font nel registro Wine
fn register_fonts_in_wine(wineprefix: &str, font_files: &[String]) {
    let runtime = detect_wine_runtime();

    for font_file in font_files {
        let font_name = if font_file.to_lowercase().contains("gothic") {
            "MS Gothic & MS PGothic & MS UI Gothic (TrueType)"
        } else if font_file.to_lowercase().contains("mincho") {
            "MS Mincho (TrueType)"
        } else if font_file.to_lowercase().contains("meiryo") {
            "Meiryo (TrueType)"
        } else if font_file.to_lowercase().contains("source") || font_file.to_lowercase().contains("han") {
            "Source Han Sans (TrueType)"
        } else {
            continue;
        };

        log_to_file(&format!("   Registering: {} → {}", font_name, font_file));

        let status = match &runtime {
            WineRuntime::ProtonExperimental(proton_path) => {
                let steam_root = find_steam_root();
                Command::new("python3")
                    .arg(proton_path)
                    .arg("run")
                    .arg("reg")
                    .arg("add")
                    .arg("HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
                    .arg("/v").arg(font_name)
                    .arg("/t").arg("REG_SZ")
                    .arg("/d").arg(font_file)
                    .arg("/f")
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
                    .arg("/v").arg(font_name)
                    .arg("/t").arg("REG_SZ")
                    .arg("/d").arg(font_file)
                    .arg("/f")
                    .env("WINEDEBUG", "-all")
                    .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
                    .status()
            }
            WineRuntime::WineSystem => {
                Command::new("wine")
                    .arg("reg").arg("add")
                    .arg("HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
                    .arg("/v").arg(font_name)
                    .arg("/t").arg("REG_SZ")
                    .arg("/d").arg(font_file)
                    .arg("/f")
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

    log_to_file("✅ Fonts registered in Wine registry");
}

pub fn run_linux(cfg: MhfConfigLinux) -> std::io::Result<()> {
    log_to_file("════════════════════════════════════════════════════");
    log_to_file("🎮 Monster Hunter Frontier Z - Linux Launcher");
    log_to_file("════════════════════════════════════════════════════");

    info!("=== Monster Hunter Frontier - Linux Launcher ===");
    debug!("Game folder: {:?}", cfg.game_folder);
    log_to_file(&format!("📁 Game folder: {:?}", cfg.game_folder));

    // Scrivi config.json
    info!("📝 Writing config.json...");
    log_to_file("📝 Writing config.json...");

    let config_path = cfg.game_folder.join("config.json");

    let notices_json: Vec<serde_json::Value> = cfg.config.notices.iter().map(|n| {
        serde_json::json!({
            "flags": n.flags,
            "data": &n.data
        })
    }).collect();

    let friends_json: Vec<serde_json::Value> = cfg.config.friends.iter().map(|f| {
        serde_json::json!({
            "cid": f.cid,
            "id": f.id,
            "name": &f.name
        })
    }).collect();

    let mez_stalls_str: Vec<String> = cfg.config.mez_stalls.iter().map(|s| {
        format!("{:?}", s)
    }).collect();

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

    info!("✅ config.json written");
    log_to_file(&format!("✅ config.json written to: {:?}", config_path));

    // XInputPlus
    log_to_file("🎮 Setting up XInputPlus for controller support...");
    info!("🎮 Setting up XInputPlus...");
    match crate::xinput::setup_xinputplus(&cfg.game_folder) {
        Ok(_) => {
            log_to_file("✅ XInputPlus configured successfully");
            info!("✅ XInputPlus configured successfully");
        }
        Err(e) => {
            log_to_file(&format!("⚠️ XInputPlus setup failed: {}", e));
            warn!("XInputPlus setup failed, controller may not work properly: {}", e);
        }
    }

    // Rileva runtime
    let runtime = detect_wine_runtime();
    log_to_file(&format!("🔧 Runtime: {:?}", runtime));

    // Cerca exe
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

    info!("Found game executable: {}", exe_name);
    log_to_file(&format!("✅ Found game executable: {}", exe_name));

    // Font config
    let fontconfig_path = env::var("FONTCONFIG_PATH")
        .unwrap_or_else(|_| "/etc/fonts".to_string());
    let fontconfig_file = env::var("FONTCONFIG_FILE")
        .unwrap_or_else(|_| "/etc/fonts/fonts.conf".to_string());
    let xdg_data_dirs = env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/share:/usr/local/share".to_string());

    // ─── Prefix ───────────────────────────────────────────────────────────────
    // Con Proton usiamo STEAM_COMPAT_DATA_PATH (es: /home/deck/MHFZ/proton_pfx)
    // Con Wine usiamo WINEPREFIX classico (es: /home/deck/MHFZ/pfx)
    let (wineprefix, compat_data_path) = match &runtime {
        WineRuntime::ProtonExperimental(_) => {
            // Proton crea pfx/ dentro compat_data_path automaticamente
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
    if let Some(ref cdp) = compat_data_path {
        log_to_file(&format!("🟣 STEAM_COMPAT_DATA_PATH: {}", cdp));
    }

    // ─── Init prefix se necessario ────────────────────────────────────────────
    let prefix_path = std::path::Path::new(&wineprefix);
    let system_reg = prefix_path.join("system.reg");
    let need_init = !prefix_path.exists() || !system_reg.exists();

    if need_init {
        log_to_file("🔧 First launch - initializing Wine/Proton prefix...");
        info!("Creating prefix (this may take 1-2 minutes on first launch)...");

        let _ = std::fs::create_dir_all(&wineprefix);

        let init_output = match &runtime {
            WineRuntime::ProtonExperimental(proton_path) => {
                let cdp = compat_data_path.as_deref().unwrap_or("");
                let steam_root = find_steam_root();
                log_to_file(&format!("🟣 Initializing Proton prefix: {}", cdp));
                Command::new("python3")
                    .arg(proton_path)
                    .arg("run")
                    .arg("wineboot")
                    .arg("--init")
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
                log_to_file("🍷 Initializing Wine Flatpak prefix...");
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
                log_to_file("🍷 Initializing system Wine prefix...");
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
            Ok(out) => {
                if out.status.success() {
                    log_to_file("✅ Prefix initialized successfully");
                    info!("Prefix initialized successfully");
                } else {
                    log_to_file(&format!("⚠️ wineboot exited: {}", out.status));
                    log_to_file(&format!("   stderr: {}", String::from_utf8_lossy(&out.stderr)));
                }
            }
            Err(e) => {
                log_to_file(&format!("❌ Failed to run wineboot: {}", e));
                error!("Failed to run wineboot: {}", e);
            }
        }

        let wait_time = 10u64;
        log_to_file(&format!("⏳ Waiting {} seconds for prefix to settle...", wait_time));
        std::thread::sleep(std::time::Duration::from_secs(wait_time));

        install_japanese_fonts(&cfg.game_folder, &wineprefix);
    } else {
        log_to_file("✅ Prefix already exists and configured");
        info!("✅ Prefix already configured");
    }

    // XAUTHORITY
    let xauthority = env::var("XAUTHORITY").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.Xauthority", home)
    });

    // ─── Lancia il gioco ──────────────────────────────────────────────────────
    info!("🚀 Starting game...");
    log_to_file("🚀 Launching game...");
    log_to_file(&format!("   Executable: {:?}", mhf_iel_exe));

    let dll_overrides = "xinput1_3=n,b;dinput=n,b;dinput8=n,b;winemenubuilder.exe=d";

    let result = match &runtime {
        WineRuntime::ProtonExperimental(proton_path) => {
            let cdp = compat_data_path.as_deref().unwrap_or("");
            let steam_root = find_steam_root();
            log_to_file(&format!("🟣 Launching via Proton Experimental: {:?}", proton_path));
            Command::new("setsid")
                .arg("python3")
                .arg(proton_path)
                .arg("run")
                .arg(&mhf_iel_exe)
                .env("STEAM_COMPAT_DATA_PATH", cdp)
                .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &steam_root)
                .env("WINEDEBUG", "-all")
                .env("WINEDLLOVERRIDES", dll_overrides)
                .env("FONTCONFIG_PATH", &fontconfig_path)
                .env("FONTCONFIG_FILE", &fontconfig_file)
                .env("XDG_DATA_DIRS", &xdg_data_dirs)
                .env("XAUTHORITY", &xauthority)
                .current_dir(&cfg.game_folder)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
        }
        WineRuntime::WineFlatpak => {
            log_to_file("🍷 Launching via Wine Flatpak (fallback)");
            Command::new("setsid")
                .arg("flatpak")
                .arg("run")
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
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
        }
        WineRuntime::WineSystem => {
            log_to_file("🍷 Launching via system Wine (fallback)");
            Command::new("setsid")
                .arg("wine")
                .arg(&mhf_iel_exe)
                .env("WINEPREFIX", &wineprefix)
                .env("WINEDEBUG", "-all")
                .env("WINEDLLOVERRIDES", dll_overrides)
                .env("FONTCONFIG_PATH", &fontconfig_path)
                .env("FONTCONFIG_FILE", &fontconfig_file)
                .env("XDG_DATA_DIRS", &xdg_data_dirs)
                .env("XAUTHORITY", &xauthority)
                .current_dir(&cfg.game_folder)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
        }
    };

    match result {
        Ok(child) => {
            log_to_file(&format!("✅ Game launched successfully (PID: {})", child.id()));
            log_to_file("🎮 Game is running!");
            log_to_file("════════════════════════════════════════════════════");
            info!("✅ Game launched successfully (PID: {})", child.id());
            info!("🎮 Game is running");
            Ok(())
        }
        Err(e) => {
            log_to_file(&format!("❌ Failed to launch game: {}", e));
            log_to_file("════════════════════════════════════════════════════");
            error!("❌ Failed to launch game: {}", e);
            Err(e)
        }
    }
}
