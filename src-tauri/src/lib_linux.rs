use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use log::{info, debug, error};
use mhf_iel::MhfConfig;

#[derive(Debug)]
pub struct MhfConfigLinux {
    pub game_folder: PathBuf,
    pub config: MhfConfig,
}

fn log_to_file(msg: &str) {
    let log_path = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()) + "/mhfz-font-debug.log";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = writeln!(file, "{}", msg);
    }
}

fn install_japanese_fonts(game_folder: &std::path::Path, wineprefix: &str) {
    let fonts_source = game_folder.join("fonts");
    if !fonts_source.exists() {
        log_to_file("⚠️ fonts/ folder not found, skipping font installation");
        return;
    }

    let fonts_dest = std::path::Path::new(wineprefix)
    .join("drive_c/windows/Fonts");

    if !fonts_dest.exists() {
        let _ = std::fs::create_dir_all(&fonts_dest);
    }

    log_to_file("🔤 Installing Japanese fonts...");
    info!("Installing Japanese fonts from fonts/ folder...");

    if let Ok(entries) = std::fs::read_dir(&fonts_source) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "ttf" || ext_str == "ttc" || ext_str == "otf" {
                    if let Some(filename) = path.file_name() {
                        let dest = fonts_dest.join(filename);
                        match std::fs::copy(&path, &dest) {
                            Ok(_) => log_to_file(&format!("   ✅ Installed: {:?}", filename)),
                            Err(e) => log_to_file(&format!("   ❌ Failed to copy {:?}: {}", filename, e)),
                        }
                    }
                }
            }
        }
    }

    log_to_file("✅ Japanese fonts installed");
    info!("Japanese fonts installation complete");
}

pub fn run_linux(cfg: MhfConfigLinux) -> std::io::Result<()> {
    info!("=== Monster Hunter Frontier - Linux Launcher ===");
    log_to_file("=== MHFZ Font Debug Log ===");

    debug!("Game folder: {:?}", cfg.game_folder);

    // ✅ CRITICAL FIX: Scrivi config.json PRIMA di lanciare il gioco
    info!("📝 Writing config.json...");
    let config_path = cfg.game_folder.join("config.json");

    // ✅ Converti manualmente i tipi per evitare problemi di serializzazione
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
        error!("❌ Failed to write config.json: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to write config.json: {}", e))
    })?;

    info!("✅ config.json written to: {:?}", config_path);
    log_to_file(&format!("✅ config.json written to: {:?}", config_path));

    // Cerca exe
    let mut mhf_iel_exe = cfg.game_folder.join("mhf-iel.exe");
    let mut exe_name = "mhf-iel.exe";

    if !mhf_iel_exe.exists() {
        mhf_iel_exe = cfg.game_folder.join("mhf-iel-cli.exe");
        exe_name = "mhf-iel-cli.exe";
    }

    if !mhf_iel_exe.exists() {
        error!("Game executable not found!");
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "mhf-iel.exe or mhf-iel-cli.exe not found in game folder"
        ));
    }

    info!("Found game executable: {}", exe_name);

    let fontconfig_path = env::var("FONTCONFIG_PATH")
    .unwrap_or_else(|_| {
        log_to_file("⚠️ FONTCONFIG_PATH NOT SET - using default");
        "/etc/fonts".to_string()
    });

    let fontconfig_file = env::var("FONTCONFIG_FILE")
    .unwrap_or_else(|_| {
        log_to_file("⚠️ FONTCONFIG_FILE NOT SET - using default");
        "/etc/fonts/fonts.conf".to_string()
    });

    let xdg_data_dirs = env::var("XDG_DATA_DIRS")
    .unwrap_or_else(|_| "/usr/share:/usr/local/share".to_string());

    log_to_file(&format!("✅ Font config loaded:"));
    log_to_file(&format!("   FONTCONFIG_PATH: {}", fontconfig_path));
    log_to_file(&format!("   FONTCONFIG_FILE: {}", fontconfig_file));
    log_to_file(&format!("   XDG_DATA_DIRS: {}", xdg_data_dirs));

    info!("Font configuration:");
    info!("  FONTCONFIG_PATH: {}", fontconfig_path);
    info!("  FONTCONFIG_FILE: {}", fontconfig_file);

    let wineprefix = env::var("WINEPREFIX").unwrap_or_else(|_| {
        let pfx_path = cfg.game_folder.join("pfx");
        let pfx_str = pfx_path.to_string_lossy().to_string();
        log_to_file(&format!("⚠️ WINEPREFIX NOT SET - calculated: {}", pfx_str));
        pfx_str
    });

    log_to_file(&format!("WINEPREFIX: {}", wineprefix));
    info!("WINEPREFIX: {}", wineprefix);

    let prefix_path = std::path::Path::new(&wineprefix);
    if !prefix_path.exists() || !prefix_path.join("system.reg").exists() {
        log_to_file(&format!("🔧 First launch detected - creating Wine prefix: {}", wineprefix));
        info!("Creating Wine prefix (this may take 1-2 minutes on first launch)...");

        let _ = std::fs::create_dir_all(&wineprefix);

        log_to_file("⏳ Running wineboot --init...");
        let status = Command::new("wineboot")
        .arg("--init")
        .env("WINEPREFIX", &wineprefix)
        .env("WINEDLLOVERRIDES", "winemenubuilder.exe=d")
        .env("WINEDEBUG", "-all")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

        match status {
            Ok(s) if s.success() => {
                log_to_file("✅ Wine prefix created successfully");
                info!("Wine prefix created successfully");
                install_japanese_fonts(&cfg.game_folder, &wineprefix);
            }
            Ok(s) => {
                log_to_file(&format!("⚠️ wineboot exited with status: {}", s));
                error!("wineboot failed with status: {}", s);
            }
            Err(e) => {
                log_to_file(&format!("❌ Failed to run wineboot: {}", e));
                error!("Failed to run wineboot: {}", e);
            }
        }
    } else {
        log_to_file("✅ Wine prefix already exists");
    }

    let xauthority = env::var("XAUTHORITY").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let xa = format!("{}/.Xauthority", home);
        log_to_file(&format!("⚠️ XAUTHORITY NOT SET - using: {}", xa));
        xa
    });

    log_to_file(&format!("XAUTHORITY: {}", xauthority));

    if std::path::Path::new(&xauthority).exists() {
        log_to_file("✅ XAUTHORITY file EXISTS");
    } else {
        log_to_file("❌ XAUTHORITY file NOT FOUND!");
    }

    debug!("Initializing Wine prefix...");
    let _ = Command::new("wineserver")
    .arg("-w")
    .env("WINEPREFIX", &wineprefix)
    .env("FONTCONFIG_PATH", &fontconfig_path)
    .env("FONTCONFIG_FILE", &fontconfig_file)
    .env("XDG_DATA_DIRS", &xdg_data_dirs)
    .env("WINEDEBUG", "-all")
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn();

    std::thread::sleep(std::time::Duration::from_secs(1));

    info!("🚀 Starting game via Wine...");
    log_to_file("🚀 Launching Wine with fontconfig variables...");

    let result = Command::new("setsid")
    .arg("wine")
    .arg(&mhf_iel_exe)
    .current_dir(&cfg.game_folder)
    .env("WINEDEBUG", "-all")
    .env("WINEPREFIX", &wineprefix)
    .env("FONTCONFIG_PATH", &fontconfig_path)
    .env("FONTCONFIG_FILE", &fontconfig_file)
    .env("XDG_DATA_DIRS", &xdg_data_dirs)
    .env("XAUTHORITY", &xauthority)
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn();

    match result {
        Ok(child) => {
            log_to_file(&format!("✅ Wine process spawned (PID: {})", child.id()));
            info!("✅ Game launched successfully (PID: {})", child.id());
            info!("🎮 Game is running");
            Ok(())
        }
        Err(e) => {
            log_to_file(&format!("❌ Failed to launch Wine: {}", e));
            error!("❌ Failed to launch game: {}", e);
            Err(e)
        }
    }
}
