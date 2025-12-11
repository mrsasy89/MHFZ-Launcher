// src-tauri/src/settings.rs

use serde_json::Value;

// ----- API pubblica usata dal resto del codice -----

// Su Windows riesporta le implementazioni reali dal modulo windows_settings.
#[cfg(target_os = "windows")]
pub use windows_settings::*;

// Su Linux forniamo una versione minimale e portabile.
#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
pub fn get_settings(_path: &std::path::Path) -> Settings {
    // Per ora usiamo solo i valori di default.
    Settings {
        hd_version: true,
        fullscreen: false,
        fullscreen_w: 1920,
        fullscreen_h: 1080,
        window_w: 1280,
        window_h: 720,
        sound: 100,
        sound_unfocused: 100,
        sound_minimized: 100,
    }
}

#[cfg(target_os = "linux")]
pub fn set_setting(
    _path: &std::path::Path,
    _name: &str,
    _value: Value,
) -> Result<(), String> {
    // Su Linux per ora non scriviamo nel file ini.
    Ok(())
}

// ----- Implementazione completa solo per Windows -----

#[cfg(target_os = "windows")]
mod windows_settings {
    use std::fs;
    use std::path::{Path, PathBuf};

    use log::warn;
    use serde::Serialize;
    use serde_json::Value;
    use windows::core::{w, HSTRING, PCWSTR};
    use windows::Win32::System::WindowsProgramming::{
        GetPrivateProfileIntW, WritePrivateProfileStringW,
    };

    #[derive(Debug, Serialize)]
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

    /// Trova il file INI nella cartella data.
    /// Se esiste mhf.ini lo usa; altrimenti il primo .ini trovato; se niente, ritorna mhf.ini.
    fn find_ini_file(folder: &Path) -> PathBuf {
        let default = folder.join("mhf.ini");
        if default.exists() {
            return default;
        }
        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().map_or(false, |ext| ext == "ini") {
                    return p;
                }
            }
        }
        default
    }

    pub fn get_settings(path: &Path) -> Settings {
        let ini_path = find_ini_file(path);
        let ini_file = HSTRING::from(ini_path.as_os_str());
        let ini_file = PCWSTR(ini_file.as_ptr());
        unsafe {
            Settings {
                hd_version: GetPrivateProfileIntW(w!("VIDEO"), w!("GRAPHICS_VER"), 1, ini_file) > 0,
                fullscreen: GetPrivateProfileIntW(w!("SCREEN"), w!("FULLSCREEN_MODE"), 1, ini_file)
                > 0,
                fullscreen_w: GetPrivateProfileIntW(
                    w!("SCREEN"),
                                                    w!("FULLSCREEN_RESOLUTION_W"),
                                                    1920,
                                                    ini_file,
                ),
                fullscreen_h: GetPrivateProfileIntW(
                    w!("SCREEN"),
                                                    w!("FULLSCREEN_RESOLUTION_H"),
                                                    1080,
                                                    ini_file,
                ),
                window_w: GetPrivateProfileIntW(
                    w!("SCREEN"),
                                                w!("WINDOW_RESOLUTION_W"),
                                                1920,
                                                ini_file,
                ),
                window_h: GetPrivateProfileIntW(
                    w!("SCREEN"),
                                                w!("WINDOW_RESOLUTION_H"),
                                                1080,
                                                ini_file,
                ),
                sound: GetPrivateProfileIntW(w!("SOUND"), w!("SOUND_VOLUME"), 0, ini_file) as u8,
                sound_unfocused: GetPrivateProfileIntW(
                    w!("SOUND"),
                                                       w!("SOUND_VOLUME_INACTIVITY"),
                                                       0,
                                                       ini_file,
                ) as u8,
                sound_minimized: GetPrivateProfileIntW(
                    w!("SOUND"),
                                                       w!("SOUND_VOLUME_MINIMIZE"),
                                                       0,
                                                       ini_file,
                ) as u8,
            }
        }
    }

    const FALSE_VALUE: PCWSTR = w!("0");
    const TRUE_VALUE: PCWSTR = w!("1");

    fn w_bool(value: bool) -> PCWSTR {
        if value {
            TRUE_VALUE
        } else {
            FALSE_VALUE
        }
    }

    fn w_string(value: String) -> PCWSTR {
        PCWSTR(HSTRING::from(value).as_ptr())
    }

    pub fn set_setting(path: &Path, name: &str, value: Value) -> Result<(), String> {
        let ini_path = find_ini_file(path);
        let ini_file = HSTRING::from(ini_path.as_os_str());
        println!("INI FILE: {:?}", ini_path);
        let ini_file = PCWSTR(ini_file.as_ptr());
        unsafe {
            match (name, value) {
                ("hdVersion", Value::Bool(v)) => {
                    WritePrivateProfileStringW(w!("VIDEO"), w!("GRAPHICS_VER"), w_bool(v), ini_file)
                }
                ("fullscreen", Value::Bool(v)) => WritePrivateProfileStringW(
                    w!("SCREEN"),
                                                                             w!("FULLSCREEN_MODE"),
                                                                             w_bool(v),
                                                                             ini_file,
                ),
                ("fullscreenW", Value::Number(n)) => WritePrivateProfileStringW(
                    w!("SCREEN"),
                                                                                w!("FULLSCREEN_RESOLUTION_W"),
                                                                                w_string(n.to_string()),
                                                                                ini_file,
                ),
                ("fullscreenH", Value::Number(n)) => WritePrivateProfileStringW(
                    w!("SCREEN"),
                                                                                w!("FULLSCREEN_RESOLUTION_H"),
                                                                                w_string(n.to_string()),
                                                                                ini_file,
                ),
                ("windowW", Value::Number(n)) => WritePrivateProfileStringW(
                    w!("SCREEN"),
                                                                            w!("WINDOW_RESOLUTION_W"),
                                                                            w_string(n.to_string()),
                                                                            ini_file,
                ),
                ("windowH", Value::Number(n)) => WritePrivateProfileStringW(
                    w!("SCREEN"),
                                                                            w!("WINDOW_RESOLUTION_H"),
                                                                            w_string(n.to_string()),
                                                                            ini_file,
                ),
                ("sound", Value::Number(n)) => WritePrivateProfileStringW(
                    w!("SOUND"),
                                                                          w!("SOUND_VOLUME"),
                                                                          w_string(n.to_string()),
                                                                          ini_file,
                ),
                ("soundUnfocused", Value::Number(n)) => WritePrivateProfileStringW(
                    w!("SOUND"),
                                                                                   w!("SOUND_VOLUME_INACTIVITY"),
                                                                                   w_string(n.to_string()),
                                                                                   ini_file,
                ),
                ("soundMinimized", Value::Number(n)) => WritePrivateProfileStringW(
                    w!("SOUND"),
                                                                                   w!("SOUND_VOLUME_MINIMIZE"),
                                                                                   w_string(n.to_string()),
                                                                                   ini_file,
                ),
                _ => {
                    warn!("unknown setting: {}", name);
                    Ok(())
                }
            }
        }
        .map_err(|e| {
            warn!("failed to write to config: {}, {}", name, e);
            "settings-error".to_owned()
        })
    }
}
