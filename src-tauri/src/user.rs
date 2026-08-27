use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::endpoint::Endpoint;
use log::{info, warn};
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "mhf-launcher";
const PASSWORDS_FILE: &str = "passwords.json";

#[derive(Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub username: String,
    pub remember_me: bool,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct UserManager {
    data: [HashMap<String, UserData>; 2],
}

impl UserManager {
    fn get_target(&self, endpoint: &'_ Endpoint) -> String {
        format!("{}:{}", endpoint.name, endpoint.is_remote)
    }

    fn passwords_file_path() -> Option<PathBuf> {
        dirs::config_dir().map(|path| path.join(APP_NAME).join(PASSWORDS_FILE))
    }

    fn read_fallback_passwords() -> HashMap<String, String> {
        let Some(path) = Self::passwords_file_path() else {
            warn!("cannot determine config directory for password fallback");
            return HashMap::new();
        };

        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|error| {
                warn!(
                    "cannot parse fallback password file {}: {}",
                    path.display(),
                      error
                );
                HashMap::new()
            }),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
            Err(error) => {
                warn!(
                    "cannot read fallback password file {}: {}",
                    path.display(),
                      error
                );
                HashMap::new()
            }
        }
    }

    fn write_fallback_passwords(passwords: &HashMap<String, String>) {
        let Some(path) = Self::passwords_file_path() else {
            warn!("cannot determine config directory for password fallback");
            return;
        };

        let Some(parent) = path.parent() else {
            warn!("cannot determine parent directory for password fallback");
            return;
        };

        if let Err(error) = fs::create_dir_all(parent) {
            warn!(
                "cannot create password fallback directory {}: {}",
                parent.display(),
                  error
            );
            return;
        }

        let content = match serde_json::to_string(passwords) {
            Ok(content) => content,
            Err(error) => {
                warn!("cannot serialize fallback passwords: {}", error);
                return;
            }
        };

        let temporary_path = path.with_extension("json.tmp");

        #[cfg(unix)]
        use std::os::unix::fs::OpenOptionsExt;

        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);

        #[cfg(unix)]
        options.mode(0o600);

        let mut file = match options.open(&temporary_path) {
            Ok(file) => file,
            Err(error) => {
                warn!(
                    "cannot open temporary fallback password file {}: {}",
                    temporary_path.display(),
                      error
                );
                return;
            }
        };

        if let Err(error) = file.write_all(content.as_bytes()) {
            warn!(
                "cannot write temporary fallback password file {}: {}",
                temporary_path.display(),
                  error
            );
            let _ = fs::remove_file(&temporary_path);
            return;
        }

        if let Err(error) = file.sync_all() {
            warn!(
                "cannot sync temporary fallback password file {}: {}",
                temporary_path.display(),
                  error
            );
            let _ = fs::remove_file(&temporary_path);
            return;
        }

        if let Err(error) = fs::rename(&temporary_path, &path) {
            warn!(
                "cannot replace fallback password file {}: {}",
                path.display(),
                  error
            );
            let _ = fs::remove_file(&temporary_path);
            return;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Err(error) = fs::set_permissions(&path, fs::Permissions::from_mode(0o600)) {
                warn!(
                    "cannot set 0600 permissions on fallback password file {}: {}",
                    path.display(),
                      error
                );
            }
        }

        info!("password saved using local fallback storage");
    }

    fn get_fallback_password(target: &str) -> String {
        Self::read_fallback_passwords()
        .remove(target)
        .unwrap_or_default()
    }

    fn set_fallback_password(target: &str, password: &str) {
        let mut passwords = Self::read_fallback_passwords();
        passwords.insert(target.to_owned(), password.to_owned());
        Self::write_fallback_passwords(&passwords);
    }

    fn delete_fallback_password(target: &str) {
        let mut passwords = Self::read_fallback_passwords();

        if passwords.remove(target).is_some() {
            Self::write_fallback_passwords(&passwords);
        }
    }

    pub fn get(&self, endpoint: &'_ Endpoint) -> (UserData, String) {
        let target = self.get_target(endpoint);
        let data = &self.data[endpoint.is_remote as usize];
        let userdata = data
        .get(&endpoint.name)
        .cloned()
        .unwrap_or_else(|| UserData {
            username: "".into(),
                        remember_me: true,
        });

        let password = if userdata.username.is_empty() {
            String::new()
        } else {
            match keyring::Entry::new_with_target(&target, APP_NAME, &userdata.username)
            .and_then(|entry| entry.get_password())
            {
                Ok(password) => password,
                Err(error) => {
                    warn!(
                        "keyring unavailable while reading password, using local fallback: {}",
                        error
                    );
                    Self::get_fallback_password(&target)
                }
            }
        };

        (userdata, password)
    }

    pub fn set(&mut self, endpoint: &'_ Endpoint, userdata: UserData, password: String) {
        let target = self.get_target(endpoint);
        let data = &mut self.data[endpoint.is_remote as usize];

        let keyring_entry =
        keyring::Entry::new_with_target(&target, APP_NAME, &userdata.username);

        if userdata.remember_me {
            match keyring_entry.and_then(|entry| entry.set_password(&password)) {
                Ok(()) => {
                    // Se in passato era stato usato il fallback, rimuove la copia locale.
                    Self::delete_fallback_password(&target);
                }
                Err(error) => {
                    warn!(
                        "keyring unavailable while saving password, using local fallback: {}",
                        error
                    );
                    Self::set_fallback_password(&target, &password);
                }
            }

            data.insert(endpoint.name.to_owned(), userdata);
        } else {
            if let Err(error) = keyring_entry.and_then(|entry| entry.delete_password()) {
                warn!("failed to delete keyring password: {}", error);
            }

            Self::delete_fallback_password(&target);
            data.remove(&endpoint.name);
        }
    }
}
