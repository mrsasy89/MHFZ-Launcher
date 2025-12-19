use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use log::{info, debug, error};

#[derive(Debug, Clone)]
pub struct MhfConfigLinux {
    pub game_folder: PathBuf,
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

    // Copia tutti i font dalla cartella fonts/
    if let Ok(entries) = std::fs::read_dir(&fonts_source) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "ttf" || ext_str == "ttc" || ext_str == "otf" {
                    if let Some(filename) = path.file_name() {
                        let dest = fonts_dest.join(filename);
                        match std::fs::copy(&path, &dest) {
                            Ok(_) => log_to_file(&format!("  ✅ Installed: {:?}", filename)),
                            Err(e) => log_to_file(&format!("  ❌ Failed to copy {:?}: {}", filename, e)),
                        }
                    }
                }
            }
        }
    }

    log_to_file("✅ Japanese fonts installed");
    info!("Japanese fonts installation complete");
}

pub fn run_linux(config: MhfConfigLinux) -> std::io::Result<()> {
    info!("=== Monster Hunter Frontier - Linux Launcher ===");
    log_to_file("=== MHFZ Font Debug Log ===");
    debug!("Game folder: {:?}", config.game_folder);

    // Cerca exe
    let mut mhf_iel_exe = config.game_folder.join("mhf-iel.exe");
    let mut exe_name = "mhf-iel.exe";

    if !mhf_iel_exe.exists() {
        mhf_iel_exe = config.game_folder.join("mhf-iel-cli.exe");
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

    // Leggi variabili fontconfig
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
    log_to_file(&format!("  FONTCONFIG_PATH: {}", fontconfig_path));
    log_to_file(&format!("  FONTCONFIG_FILE: {}", fontconfig_file));
    log_to_file(&format!("  XDG_DATA_DIRS: {}", xdg_data_dirs));

    info!("Font configuration:");
    info!("  FONTCONFIG_PATH: {}", fontconfig_path);
    info!("  FONTCONFIG_FILE: {}", fontconfig_file);

    // ✅ CRITICAL: pfx nella stessa cartella del gioco
    let wineprefix = env::var("WINEPREFIX").unwrap_or_else(|_| {
        let pfx_path = config.game_folder.join("pfx");
        let pfx_str = pfx_path.to_string_lossy().to_string();
        log_to_file(&format!("⚠️ WINEPREFIX NOT SET - calculated: {}", pfx_str));
        pfx_str
    });

    log_to_file(&format!("WINEPREFIX: {}", wineprefix));
    info!("WINEPREFIX: {}", wineprefix);

    // ✅ NUOVO: Crea prefix se non esiste
    let prefix_path = std::path::Path::new(&wineprefix);
    if !prefix_path.exists() || !prefix_path.join("system.reg").exists() {
        log_to_file(&format!("🔧 First launch detected - creating Wine prefix: {}", wineprefix));
        info!("Creating Wine prefix (this may take 1-2 minutes on first launch)...");

        // Crea directory se non esiste
        let _ = std::fs::create_dir_all(&wineprefix);

        // Inizializza Wine prefix
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

                // Installa font giapponesi
                install_japanese_fonts(&config.game_folder, &wineprefix);
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

    // Leggi XAUTHORITY
    let xauthority = env::var("XAUTHORITY").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let xa = format!("{}/.Xauthority", home);
        log_to_file(&format!("⚠️ XAUTHORITY NOT SET - using: {}", xa));
        xa
    });

    log_to_file(&format!("XAUTHORITY: {}", xauthority));

    // Verifica se XAUTHORITY esiste
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
    .current_dir(&config.game_folder)
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
            log_to_file(&format!("✅ Wine has WINEPREFIX={}", wineprefix));
            log_to_file(&format!("✅ Wine has XAUTHORITY={}", xauthority));
            log_to_file(&format!("✅ Wine has FONTCONFIG_PATH={}", fontconfig_path));
            log_to_file(&format!("✅ Wine has FONTCONFIG_FILE={}", fontconfig_file));

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
