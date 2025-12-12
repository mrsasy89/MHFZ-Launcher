use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct MhfConfigLinux {
    pub game_folder: PathBuf,
}

/// Struct per passare dati mhf-iel (da main.rs)
pub struct MhfIelConfig {
    pub char_id: u32,
    pub char_name: String,
    pub char_hr: u32,
    pub char_gr: u32,
    pub char_ids: Vec<u32>,
    pub char_new: bool,
    pub user_token_id: u32,
    pub user_token: String,
    pub username: String,
    pub user_password: String,
    pub user_rights: u32,
    pub server_host: String,
    pub server_port: u32,
    pub entrance_count: u32,
    pub current_ts: u64,
    pub expiry_ts: u64,
    pub notices: Vec<serde_json::Value>,
    pub mez_event_id: u32,
    pub mez_start: u64,
    pub mez_end: u64,
    pub mez_solo_tickets: u32,
    pub mez_group_tickets: u32,
    pub mez_stalls: Vec<String>,
    pub version: String,
}

/// Genera config.json per mhf-iel-cli
fn generate_mhf_iel_config(
    game_folder: &PathBuf,
    config: &MhfIelConfig,
) -> Result<(), String> {
    // Valida token
    if config.user_token.len() != 16 {
        return Err(format!(
            "❌ user_token deve essere 16 caratteri (ricevuto: {})",
                           config.user_token.len()
        ));
    }

    let config_json = json!({
        "char_id": config.char_id,
        "char_name": config.char_name,
        "char_new": config.char_new,
        "char_hr": config.char_hr,
        "char_gr": config.char_gr,
        "char_ids": config.char_ids,
        "user_rights": config.user_rights,
        "user_token": config.user_token,
        "user_token_id": config.user_token_id,
        "user_name": config.username,
        "user_password": config.user_password,
        "server_host": config.server_host,
        "server_port": config.server_port,
        "notices": config.notices,
        "version": match config.version.as_str() {
            "F5" => "F5",
            _ => "ZZ"
        },
        "entrance_count": config.entrance_count,
        "current_ts": config.current_ts,
        "expiry_ts": config.expiry_ts,
        "messages": [],
        "mez_event_id": config.mez_event_id,
        "mez_start": config.mez_start,
        "mez_end": config.mez_end,
        "mez_solo_tickets": config.mez_solo_tickets,
        "mez_group_tickets": config.mez_group_tickets,
        "mez_stalls": config.mez_stalls
    });

    let config_path = game_folder.join("config.json");

    fs::write(&config_path, serde_json::to_string_pretty(&config_json).unwrap())
    .map_err(|e| format!("❌ Errore config.json: {}", e))?;

    println!("✅ config.json → {:?}", config_path);
    println!("   Server: {}:{}", config.server_host, config.server_port);
    println!("   Char: {} (ID {})", config.char_name, config.char_id);

    Ok(())
}

pub fn run_linux(config: MhfConfigLinux) -> std::io::Result<()> {
    println!("🔧 Preparazione MHFZ con mhf-iel...");

    // Ottieni config da storage globale
    let iel_config = MHF_IEL_CONFIG_GLOBAL.get();

    if let Some(cfg) = iel_config {
        // Genera config.json
        if let Err(e) = generate_mhf_iel_config(&config.game_folder, cfg) {
            eprintln!("{}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }

        // Verifica mhf-iel-cli.exe
        let iel_path = config.game_folder.join("mhf-iel-cli.exe");
        if !iel_path.exists() {
            eprintln!("❌ mhf-iel-cli.exe non trovato: {:?}", iel_path);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "mhf-iel-cli.exe not found"
            ));
        }

        // Wine prefix
        let wine_prefix = config.game_folder.join("pfx");

        println!("🍷 WINEPREFIX: {:?}", wine_prefix);
        println!("🎮 Lancio gioco...");

        // Esegui mhf-iel-cli.exe
        let mut command = Command::new("wine");
        command
        .arg(&iel_path)
        .current_dir(&config.game_folder)
        .env("WINEPREFIX", &wine_prefix)
        .env("DXVK_HUD", "fps");

        command.spawn()?.wait()?;

        println!("✅ Gioco chiuso");
        Ok(())
    } else {
        // Fallback: usa mhf.exe con Proton (vecchio metodo)
        eprintln!("⚠️ mhf-iel config non disponibile, fallback a Proton");

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
}

// Global config storage (thread-safe)
static MHF_IEL_CONFIG_GLOBAL: OnceLock<MhfIelConfig> = OnceLock::new();

pub fn set_mhf_iel_config(config: MhfIelConfig) {
    let _ = MHF_IEL_CONFIG_GLOBAL.set(config);
}
