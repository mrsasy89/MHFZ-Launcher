// src-tauri/src/ini_parser.rs

use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Costante per line ending basata sulla piattaforma di compilazione
#[cfg(target_os = "windows")]
const LINE_ENDING: &str = "\r\n";

#[cfg(not(target_os = "windows"))]
const LINE_ENDING: &str = "\n";

#[derive(Debug, Clone)]
pub struct IniFile {
    sections: Vec<String>,
    data: HashMap<String, Vec<(String, String)>>,
    /// Mantiene il line ending del file originale
    line_ending: String,
}

impl IniFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
        .map_err(|e| format!("Impossibile leggere il file INI: {}", e))?;

        Self::parse(&content)
    }

    fn parse(content: &str) -> Result<Self, String> {
        let mut sections = Vec::new();
        let mut data: HashMap<String, Vec<(String, String)>> = HashMap::new();

        // Rileva il line ending del file (preserva quello originale)
        let line_ending = if content.contains("\r\n") {
            "\r\n".to_string()
        } else {
            "\n".to_string()
        };

        let mut current_section = String::new();

        // Normalizza le line endings per il parsing
        let normalized = content.replace("\r\n", "\n");

        for line in normalized.lines() {
            let trimmed = line.trim();

            // Salta righe vuote e commenti
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }

            // Sezione [NOME]
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len()-1].to_string();
                if !sections.contains(&current_section) {
                    sections.push(current_section.clone());
                }
                data.entry(current_section.clone()).or_insert_with(Vec::new);
                continue;
            }

            // Chiave=Valore
            if let Some(pos) = trimmed.find('=') {
                let key = trimmed[..pos].trim().to_string();
                let value = trimmed[pos+1..].trim().to_string();

                if !current_section.is_empty() {
                    if let Some(section_data) = data.get_mut(&current_section) {
                        section_data.push((key, value));
                    }
                }
            }
        }

        Ok(IniFile {
            sections,
            data,
            line_ending,
        })
    }

    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        self.data.get(section)?.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
    }

    pub fn set(&mut self, section: &str, key: &str, value: &str) {
        let section = section.to_string();
        let key = key.to_string();
        let value = value.to_string();

        if !self.sections.contains(&section) {
            self.sections.push(section.clone());
        }

        let section_data = self.data.entry(section.clone()).or_insert_with(Vec::new);

        if let Some(entry) = section_data.iter_mut().find(|(k, _)| k == &key) {
            entry.1 = value;
        } else {
            section_data.push((key, value));
        }
    }

    /// Salva usando il line ending originale del file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let mut output = String::new();

        for section in &self.sections {
            output.push_str(&format!("[{}]{}", section, self.line_ending));

            if let Some(pairs) = self.data.get(section) {
                for (key, value) in pairs {
                    output.push_str(&format!("{}={}{}", key, value, self.line_ending));
                }
            }
        }

        fs::write(path, output)
        .map_err(|e| format!("Impossibile salvare il file INI: {}", e))
    }

    /// Debug helper
    pub fn dump(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("Line ending: {:?}\n", self.line_ending));
        for section in &self.sections {
            result.push_str(&format!("[{}]\n", section));
            if let Some(pairs) = self.data.get(section) {
                for (key, value) in pairs {
                    result.push_str(&format!("  {} = {}\n", key, value));
                }
            }
        }
        result
    }
}

/// Applica le impostazioni del launcher al file mhf.ini
/// Cross-platform: funziona su Windows e Linux
pub fn apply_game_settings(
    ini_path: &Path,
    hd_version: bool,
    fullscreen: bool,
    window_w: u32,
    window_h: u32,
    fullscreen_w: u32,
    fullscreen_h: u32,
) -> Result<(), String> {
    // Verifica che il file esista
    if !ini_path.exists() {
        return Err(format!("File INI non trovato: {:?}", ini_path));
    }

    let mut ini = IniFile::from_file(ini_path)?;

    // [VIDEO] GRAPHICS_VER (HD)
    ini.set("VIDEO", "GRAPHICS_VER", if hd_version { "1" } else { "0" });

    // [SCREEN] FULLSCREEN_MODE
    ini.set("SCREEN", "FULLSCREEN_MODE", if fullscreen { "1" } else { "0" });

    // [SCREEN] Risoluzioni
    ini.set("SCREEN", "WINDOW_RESOLUTION_W", &window_w.to_string());
    ini.set("SCREEN", "WINDOW_RESOLUTION_H", &window_h.to_string());
    ini.set("SCREEN", "FULLSCREEN_RESOLUTION_W", &fullscreen_w.to_string());
    ini.set("SCREEN", "FULLSCREEN_RESOLUTION_H", &fullscreen_h.to_string());

    ini.save(ini_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_windows_line_endings() {
        let content = "[SCREEN]\r\nFULLSCREEN_MODE=1\r\nWINDOW_RESOLUTION_W=1280\r\n";
        let ini = IniFile::parse(content).unwrap();

        assert_eq!(ini.line_ending, "\r\n");
        assert_eq!(ini.get("SCREEN", "FULLSCREEN_MODE"), Some("1".to_string()));
    }

    #[test]
    fn test_parse_unix_line_endings() {
        let content = "[SCREEN]\nFULLSCREEN_MODE=1\nWINDOW_RESOLUTION_W=1280\n";
        let ini = IniFile::parse(content).unwrap();

        assert_eq!(ini.line_ending, "\n");
        assert_eq!(ini.get("SCREEN", "FULLSCREEN_MODE"), Some("1".to_string()));
    }

    #[test]
    fn test_modify_values() {
        let content = "[VIDEO]\r\nGRAPHICS_VER=0\r\n";
        let mut ini = IniFile::parse(content).unwrap();

        ini.set("VIDEO", "GRAPHICS_VER", "1");
        assert_eq!(ini.get("VIDEO", "GRAPHICS_VER"), Some("1".to_string()));
    }
}
