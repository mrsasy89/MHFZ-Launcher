use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct MhfConfigLinux {
    pub game_folder: PathBuf,
}

pub fn run_linux(config: MhfConfigLinux) -> std::io::Result<()> {
    let proton_root = dirs::home_dir()
    .unwrap_or_else(|| PathBuf::from("."))
    .join(".local/share/Steam/compatibilitytools.d/GE-Proton10-25");

    let proton_script = proton_root.join("proton");
    let game_exe = config.game_folder.join("mhf.exe");

    let mut cmd = Command::new(&proton_script);
    cmd.current_dir(&config.game_folder)
    .arg("run")
    .arg(&game_exe)
    .env("STEAM_COMPAT_DATA_PATH", &config.game_folder)
    .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &proton_root)
    .env("WINEDEBUG", "-all");

    cmd.spawn()?.wait()?;
    Ok(())
}
