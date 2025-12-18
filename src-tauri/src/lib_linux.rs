use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use log::{info, debug, error, warn};

#[derive(Debug, Clone)]
pub struct MhfConfigLinux {
    pub game_folder: PathBuf,
}

pub fn run_linux(config: MhfConfigLinux) -> std::io::Result<()> {
    info!("=== Monster Hunter Frontier - Linux Launcher ===");
    debug!("Game folder: {:?}", config.game_folder);

    // Cerca prima mhf-iel.exe, poi mhf-iel-cli.exe
    let mut mhf_iel_exe = config.game_folder.join("mhf-iel.exe");
    let mut exe_name = "mhf-iel.exe";

    if !mhf_iel_exe.exists() {
        mhf_iel_exe = config.game_folder.join("mhf-iel-cli.exe");
        exe_name = "mhf-iel-cli.exe";
    }

    debug!("Looking for executable: {}", exe_name);
    debug!("Full path: {:?}", mhf_iel_exe);

    if !mhf_iel_exe.exists() {
        error!("Game executable not found!");
        error!("Searched for: mhf-iel.exe and mhf-iel-cli.exe");
        error!("In directory: {:?}", config.game_folder);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "mhf-iel.exe or mhf-iel-cli.exe not found in game folder"
        ));
    }

    info!("Found game executable: {}", exe_name);

    // ✅ FIX: Leggi variabili d'ambiente O usa fallback hardcoded
    let fontconfig_path = env::var("FONTCONFIG_PATH")
    .unwrap_or_else(|_| {
        debug!("FONTCONFIG_PATH not set, using fallback: /etc/fonts");
        "/etc/fonts".to_string()
    });

    let fontconfig_file = env::var("FONTCONFIG_FILE")
    .unwrap_or_else(|_| {
        debug!("FONTCONFIG_FILE not set, using fallback: /etc/fonts/fonts.conf");
        "/etc/fonts/fonts.conf".to_string()
    });

    let xdg_data_dirs = env::var("XDG_DATA_DIRS")
    .unwrap_or_else(|_| {
        debug!("XDG_DATA_DIRS not set, using fallback");
        "/usr/share:/usr/local/share".to_string()
    });

    info!("Font configuration:");
    info!("  FONTCONFIG_PATH: {}", fontconfig_path);
    info!("  FONTCONFIG_FILE: {}", fontconfig_file);
    info!("  XDG_DATA_DIRS: {}", xdg_data_dirs);

    // 🔧 CRITICAL FIX: Pre-inizializza Wine prefix con le variabili font!
    info!("🔧 Pre-initializing Wine prefix with font configuration...");

    let wineprefix = env::var("WINEPREFIX")
    .unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.wine", home)
    });

    debug!("Using WINEPREFIX: {}", wineprefix);

    // Lancia wineboot per inizializzare/aggiornare il prefix
    let wineboot_result = Command::new("wineboot")
    .arg("-i")  // prefix
    .env("WINEPREFIX", &wineprefix)
    .env("FONTCONFIG_PATH", &fontconfig_path)
    .env("FONTCONFIG_FILE", &fontconfig_file)
    .env("XDG_DATA_DIRS", &xdg_data_dirs)
    .env("WINEDEBUG", "-all")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status();

    match wineboot_result {
        Ok(status) => {
            if status.success() {
                info!("✅ Wine prefix initialized successfully with font configuration");
            } else {
                warn!("⚠️  wineboot returned non-zero status, but continuing...");
            }
        }
        Err(e) => {
            warn!("⚠️  wineboot failed: {}, but continuing...", e);
        }
    }

    info!("🚀 Starting game via Wine...");

    // ✅ Costruisci comando Wine con tutte le variabili
    let mut cmd = Command::new("wine");
    cmd.current_dir(&config.game_folder)
    .arg(&mhf_iel_exe)
    .env("WINEDEBUG", "-all")
    .env("WINEPREFIX", &wineprefix)
    // ✅ CRITICAL: Passa esplicitamente le variabili a Wine
    .env("FONTCONFIG_PATH", &fontconfig_path)
    .env("FONTCONFIG_FILE", &fontconfig_file)
    .env("XDG_DATA_DIRS", &xdg_data_dirs);

    debug!("Command: wine {:?}", mhf_iel_exe);
    debug!("Working directory: {:?}", config.game_folder);
    debug!("Environment variables set for Wine process:");
    debug!("  WINEPREFIX={}", wineprefix);
    debug!("  FONTCONFIG_PATH={}", fontconfig_path);
    debug!("  FONTCONFIG_FILE={}", fontconfig_file);
    debug!("  XDG_DATA_DIRS={}", xdg_data_dirs);

    info!("Launching game and closing launcher...");

    // Lancia il gioco in background (NON aspettare!)
    match cmd.spawn() {
        Ok(_child) => {
            info!("✅ Game process started successfully");
            info!("Launcher will now close");
            Ok(())
        }
        Err(e) => {
            error!("❌ Failed to launch game: {}", e);
            Err(e)
        }
    }
}
