use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use log::{info, debug, error, warn};

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

    // ✅ LOGGING CRITICO: Verifica variabili d'ambiente
    let fontconfig_path = env::var("FONTCONFIG_PATH")
    .unwrap_or_else(|_| {
        log_to_file("⚠️ FONTCONFIG_PATH NOT SET!");
        "/etc/fonts".to_string()
    });

    let fontconfig_file = env::var("FONTCONFIG_FILE")
    .unwrap_or_else(|_| {
        log_to_file("⚠️ FONTCONFIG_FILE NOT SET!");
        "/etc/fonts/fonts.conf".to_string()
    });

    let xdg_data_dirs = env::var("XDG_DATA_DIRS")
    .unwrap_or_else(|_| "/usr/share:/usr/local/share".to_string());

    log_to_file(&format!("FONTCONFIG_PATH: {}", fontconfig_path));
    log_to_file(&format!("FONTCONFIG_FILE: {}", fontconfig_file));
    log_to_file(&format!("XDG_DATA_DIRS: {}", xdg_data_dirs));

    info!("Font configuration:");
    info!("  FONTCONFIG_PATH: {}", fontconfig_path);
    info!("  FONTCONFIG_FILE: {}", fontconfig_file);

    let wineprefix = env::var("WINEPREFIX").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.wine", home)
    });

    log_to_file(&format!("WINEPREFIX: {}", wineprefix));
    info!("WINEPREFIX: {}", wineprefix);

    // ✅ CRITICAL: Verifica se siamo in un terminale
    use std::io::IsTerminal;
    let is_terminal = std::io::stdin().is_terminal();
    log_to_file(&format!("Is terminal (stdin): {}", is_terminal));

    let term_env = env::var("TERM").unwrap_or_else(|_| "NONE".to_string());
    log_to_file(&format!("TERM env var: {}", term_env));

    debug!("Initializing Wine prefix...");

    let _wineserver = Command::new("wineserver")
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
    log_to_file("🚀 Launching Wine process...");

    let mut cmd = Command::new("wine");
    cmd.current_dir(&config.game_folder)
    .arg(&mhf_iel_exe)
    .env("WINEDEBUG", "-all")
    .env("WINEPREFIX", &wineprefix)
    .env("FONTCONFIG_PATH", &fontconfig_path)
    .env("FONTCONFIG_FILE", &fontconfig_file)
    .env("XDG_DATA_DIRS", &xdg_data_dirs);

    log_to_file(&format!("Wine command: wine {:?}", mhf_iel_exe));
    log_to_file(&format!("Wine WINEPREFIX: {}", wineprefix));
    log_to_file(&format!("Wine FONTCONFIG_PATH: {}", fontconfig_path));
    log_to_file(&format!("Wine FONTCONFIG_FILE: {}", fontconfig_file));

    info!("Launching game and closing launcher...");

    match cmd.spawn() {
        Ok(_child) => {
            log_to_file("✅ Game process started");
            info!("✅ Game process started successfully");
            Ok(())
        }
        Err(e) => {
            log_to_file(&format!("❌ Failed to launch: {}", e));
            error!("❌ Failed to launch game: {}", e);  // ✅ FIX: rimossa parentesi
            Err(e)
        }
    }
}
