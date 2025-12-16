// src-tauri/src/settings.rs

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

// Usa il nostro parser INI custom su entrambi i sistemi
use crate::ini_parser;

// ----- Struttura Settings unificata per Windows e Linux -----
#[derive(Debug, serde::Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub hd_version: bool,
    pub fullscreen: bool,
    pub fullscreen_w: i32,
    pub fullscreen_h: i32,
    pub window_w: i32,
    pub window_h: i32,
    pub sound: u8,
    pub sound_unfocused: u8,
    pub sound_minimized: u8,
}

/// Trova il file INI nella cartella data
fn find_ini_file(folder: &Path) -> PathBuf {
    let default = folder.join("mhf.ini");

    // Se esiste mhf.ini, usalo
    if default.exists() {
        log::info!("📄 Trovato mhf.ini in {:?}", default);
        return default;
    }

    // Altrimenti cerca qualsiasi .ini
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().map_or(false, |ext| ext == "ini") {
                log::info!("📄 Trovato file INI alternativo: {:?}", p);
                return p;
            }
        }
    }

    // Se non trova niente, ritorna il default (verrà creato)
    log::warn!("⚠️ Nessun file INI trovato, verrà creato: {:?}", default);
    default
}

/// Legge le impostazioni dal file mhf.ini
pub fn get_settings(path: &Path) -> Settings {
    let ini_path = find_ini_file(path);

    log::info!("🔍 Lettura impostazioni da: {:?}", ini_path);

    match ini_parser::IniFile::from_file(&ini_path) {
        Ok(ini) => {
            let settings = Settings {
                hd_version: ini.get("VIDEO", "GRAPHICS_VER")
                .and_then(|v| v.parse::<i32>().ok())
                .map(|v| v > 0)
                .unwrap_or(true),

                fullscreen: ini.get("SCREEN", "FULLSCREEN_MODE")
                .and_then(|v| v.parse::<i32>().ok())
                .map(|v| v > 0)
                .unwrap_or(false),

                fullscreen_w: ini.get("SCREEN", "FULLSCREEN_RESOLUTION_W")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1920),

                fullscreen_h: ini.get("SCREEN", "FULLSCREEN_RESOLUTION_H")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1080),

                window_w: ini.get("SCREEN", "WINDOW_RESOLUTION_W")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1280),

                window_h: ini.get("SCREEN", "WINDOW_RESOLUTION_H")
                .and_then(|v| v.parse().ok())
                .unwrap_or(720),

                sound: ini.get("SOUND", "SOUND_VOLUME")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),

                sound_unfocused: ini.get("SOUND", "SOUND_VOLUME_INACTIVITY")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),

                sound_minimized: ini.get("SOUND", "SOUND_VOLUME_MINIMIZE")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            };

            log::info!("✅ Impostazioni caricate: HD={}, Fullscreen={}, WinRes={}x{}",
                       settings.hd_version, settings.fullscreen,
                       settings.window_w, settings.window_h);

            settings
        }
        Err(e) => {
            log::warn!("⚠️ Impossibile leggere mhf.ini: {}, usando defaults", e);
            Settings::default()
        }
    }
}

/// Scrive una singola impostazione nel file mhf.ini
pub fn set_setting(
    path: &Path,
    name: &str,
    value: Value,
) -> Result<(), String> {
    let ini_path = find_ini_file(path);

    log::info!("💾 Salvataggio setting '{}' = {:?} in {:?}", name, value, ini_path);

    // Se il file non esiste, crea uno schema minimo
    if !ini_path.exists() {
        log::info!("📝 Creazione nuovo file INI con defaults");
        create_default_ini(&ini_path)?;
    }

    // Carica il file INI esistente
    let mut ini = ini_parser::IniFile::from_file(&ini_path)
    .map_err(|e| {
        log::error!("❌ Errore lettura INI: {}", e);
        format!("settings-read-error: {}", e)
    })?;

    // Applica la modifica in base al nome del setting
    match (name, value) {
        ("hdVersion", Value::Bool(v)) => {
            ini.set("VIDEO", "GRAPHICS_VER", if v { "1" } else { "0" });
        }
        ("fullscreen", Value::Bool(v)) => {
            ini.set("SCREEN", "FULLSCREEN_MODE", if v { "1" } else { "0" });
        }
        ("fullscreenW", Value::Number(n)) => {
            ini.set("SCREEN", "FULLSCREEN_RESOLUTION_W", &n.to_string());
        }
        ("fullscreenH", Value::Number(n)) => {
            ini.set("SCREEN", "FULLSCREEN_RESOLUTION_H", &n.to_string());
        }
        ("windowW", Value::Number(n)) => {
            ini.set("SCREEN", "WINDOW_RESOLUTION_W", &n.to_string());
        }
        ("windowH", Value::Number(n)) => {
            ini.set("SCREEN", "WINDOW_RESOLUTION_H", &n.to_string());
        }
        ("sound", Value::Number(n)) => {
            ini.set("SOUND", "SOUND_VOLUME", &n.to_string());
        }
        ("soundUnfocused", Value::Number(n)) => {
            ini.set("SOUND", "SOUND_VOLUME_INACTIVITY", &n.to_string());
        }
        ("soundMinimized", Value::Number(n)) => {
            ini.set("SOUND", "SOUND_VOLUME_MINIMIZE", &n.to_string());
        }
        _ => {
            log::warn!("⚠️ Setting sconosciuto ignorato: {}", name);
            return Ok(());
        }
    }

    // Salva il file modificato
    ini.save(&ini_path).map_err(|e| {
        log::error!("❌ Errore salvataggio INI: {}", e);
        format!("settings-write-error: {}", e)
    })?;

    log::info!("✅ Setting '{}' salvato con successo!", name);
    Ok(())
}

/// Crea un file INI con valori di default minimi
fn create_default_ini(path: &PathBuf) -> Result<(), String> {
    let default_content = "\
[SCREEN]
FULLSCREEN_MODE=0
WINDOW_RESOLUTION_W=1280
WINDOW_RESOLUTION_H=720
FULLSCREEN_RESOLUTION_W=1920
FULLSCREEN_RESOLUTION_H=1080

[VIDEO]
GRAPHICS_VER=1

[SOUND]
SOUND_VOLUME=0
SOUND_VOLUME_INACTIVITY=0
SOUND_VOLUME_MINIMIZE=0
";

// Crea la directory se non esiste
if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)
    .map_err(|e| format!("Impossibile creare directory: {}", e))?;
}

fs::write(path, default_content)
.map_err(|e| format!("Impossibile creare file INI: {}", e))?;

log::info!("✅ File INI creato con defaults in {:?}", path);
Ok(())
}
