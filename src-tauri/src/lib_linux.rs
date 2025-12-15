use std::path::PathBuf;
use std::process::Command;
use log::{info, debug, error};

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
    info!("Starting game via Wine...");

    // Usa Wine direttamente, NON Proton
    let mut cmd = Command::new("wine");
    cmd.current_dir(&config.game_folder)
    .arg(&mhf_iel_exe)
    .env("WINEDEBUG", "-all");  // Disabilita output verboso di Wine

    debug!("Command: wine {:?}", mhf_iel_exe);
    debug!("Working directory: {:?}", config.game_folder);
    debug!("Environment: WINEDEBUG=-all");

    info!("Game process starting...");

    // Avvia il processo e attendi la chiusura
    let mut child = cmd.spawn()?;
    let status = child.wait()?;

    // Log della chiusura
    if status.success() {
        info!("Game closed successfully");
        info!("Exit code: 0");
    } else {
        match status.code() {
            Some(code) => {
                error!("Game closed with error code: {}", code);
            }
            None => {
                error!("Game process terminated by signal");
            }
        }
    }

    info!("=== Launcher shutdown complete ===");

    Ok(())
}
