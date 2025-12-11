use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use log::{info, warn};
use serde::Serialize;
use serde_repr::Serialize_repr;
use sha2::Digest;
use tauri::Window;
use tokio::select;
use tokio_util::sync::CancellationToken;
use reqwest;
use crate::manifest::Manifest;
use crate::{server::PatcherResponse, LogPayload};

pub const NETWORK_ERROR: &str = "patcher-network-error";
pub const FILE_ERROR: &str = "patcher-file-error";
const ACTIVE_SERVER_FILE: &str = "ButterClient/active_server";
/// New: client version file
const CLIENT_VERSION_FILE: &str = "ButterVersion.txt";

/// foo/bar.txt → foo/bar.txt.butterold
fn backup_name(p: &Path) -> PathBuf {
    let mut f = p.file_name().unwrap().to_os_string();
    f.push(".butterold");
    p.with_file_name(f)
}

/// If `target` exists, rename to `*.butterold`.  
/// Returns **true** if backed up an existing file (i.e. it counts as “modified”).
fn backup_original(target: &Path) -> io::Result<bool> {
    if target.exists() {
        fs::rename(target, backup_name(target))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Debug, Serialize_repr, Clone)]
#[repr(u8)]
enum State {
    Checking,
    Downloading,
    Patching,
    Done,
    Error,
}

#[derive(Debug, Clone, Serialize)]
struct PatcherEvent {
    total: usize,
    current: usize,
    state: State,
}

fn send_event(window: &Window, total: usize, current: usize, state: State) {
    window
        .emit(
            "patcher",
            PatcherEvent {
                total,
                current,
                state,
            },
        )
        .unwrap_or_else(|e| warn!("failed to emit message: {}", e));
}

fn send_error(window: &Window, msg: &str) {
    warn!("patcher error: {}", msg);
    window
        .emit("log", LogPayload::error(msg))
        .unwrap_or_else(|e| warn!("failed to emit message: {}", e));
    window
        .emit(
            "patcher",
            PatcherEvent {
                total: 0,
                current: 0,
                state: State::Error,
            },
        )
        .unwrap_or_else(|e| warn!("failed to emit message: {}", e));
}

fn get_changed_paths<'a>(
    patcher_content: &'a str,
    game_folder: &Path,
) -> Result<Vec<&'a str>, &'static str> {
    let mut result = Vec::new();

    for line in patcher_content.lines() {
        let Some((patcher_hash, mut patcher_path)) = line.split_once('\t') else {
            return Err(NETWORK_ERROR);
        };
        patcher_path = patcher_path.trim_start_matches('/');
        let client_path = game_folder.join(patcher_path);

        info!(
            "files: {} {} {}",
            game_folder.to_str().unwrap(),
              &patcher_path,
              &client_path.to_str().unwrap()
        );

        let mut changed = true;

        if let Ok(mut file) = fs::File::open(&client_path) {
            let mut hasher = sha2::Sha256::new();
            if io::copy(&mut file, &mut hasher).is_ok() {
                let client_hash = format!("{:x}", hasher.finalize());
                info!("hashes: {} {}", patcher_hash, client_hash);
                if patcher_hash == client_hash {
                    changed = false;
                }
            }
        }

        if changed {
            result.push(patcher_path);
        }
    }

    Ok(result)
}

async fn download_changed_paths(
    window: &Window,
    client: &reqwest::Client,
    patcher_url: &str,
    changed_paths: &[&str],
    patcher_folder: &Path,
    cancel: CancellationToken,
) -> Result<(), &'static str> {
    let total = changed_paths.len();
    let mut current = 0;
    for changed_path in changed_paths {
        let req = client
            .get(format!("{}/{}", patcher_url, changed_path))
            .send();
        let mut resp = select! {
            _ = cancel.cancelled() => return Ok(()),
            resp = req => resp.or(Err(NETWORK_ERROR))?,
        };
        let patcher_path = patcher_folder.join(changed_path);
        fs::create_dir_all(patcher_path.parent().ok_or(FILE_ERROR)?).or(Err(FILE_ERROR))?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(patcher_path)
            .or(Err(FILE_ERROR))?;
        while let Some(chunk) = select! {
            _ = cancel.cancelled() => return Ok(()),
            chunk = resp.chunk() => chunk.or(Err(NETWORK_ERROR))?
        } {
            file.write_all(&chunk).or(Err(NETWORK_ERROR))?;
        }
        current += 1;
        send_event(window, total, current, State::Downloading);
    }
    Ok(())
}

fn move_changed_paths(
    changed_paths: &[&str],
    source_folder: &Path,
    target_folder: &Path,
    manifest: &mut Manifest,          // ← new
) -> Result<(), &'static str> {
    for rel in changed_paths {
        let source = source_folder.join(rel);
        let target = target_folder.join(rel);

        fs::create_dir_all(
            target.parent().ok_or(FILE_ERROR)?
        ).or(Err(FILE_ERROR))?;

        match backup_original(&target) {
            Ok(true)  => manifest.modified_files.push(rel.to_string()),
            Ok(false) => manifest.added_files.push(rel.to_string()),
            Err(_)    => return Err(FILE_ERROR),
        }

        fs::rename(&source, &target).or(Err(FILE_ERROR))?;
    }
    Ok(())
}

/// Restore every file that was changed by `server`.
pub fn restore_server(root: &Path, server: &str) -> io::Result<()> {
    let manifest = Manifest::load(root, server);

    for rel in manifest.modified_files {
        let orig = root.join(&rel);
        let bak  = backup_name(&orig);
        if bak.exists() {
            let _ = fs::remove_file(&orig);
            fs::rename(bak, orig)?;
        }
    }
    for rel in manifest.added_files {
        let _ = fs::remove_file(root.join(&rel));
    }

    Manifest::delete(root, server);
    Ok(())
}

async fn patch_internal(
    window: &Window,
    client: reqwest::Client,
    patcher_url: String,
    patcher_resp: PatcherResponse,
    game_folder: &Path,
    patcher_folder: &Path,
    cancel: CancellationToken,
) -> Result<(), &'static str> {
    // ─── 1. compare hashes ───────────────────────────────────────────────
    send_event(window, 0, 0, State::Checking);
    let changed_paths = get_changed_paths(&patcher_resp.content, game_folder)?;
    send_event(window, changed_paths.len(), 0, State::Downloading);

    // ─── 2. download the delta files to <game>/tmp/ ──────────────────────
    download_changed_paths(
        window,
        &client,
        &patcher_url,
        &changed_paths,
        patcher_folder,
        cancel.clone(),
    )
    .await?;

    // ─── 3. patch in-place, writing a manifest ───────────────────────────
    send_event(window, 0, 0, State::Patching);

    let mut manifest = Manifest::default();
    move_changed_paths(
        &changed_paths,
        patcher_folder,
        game_folder,
        &mut manifest,
    )?;

    manifest
        .save(game_folder, &patcher_resp.server_name)
        .unwrap_or_else(|e| warn!("manifest save failed: {}", e));

    // ─── 4. done ─────────────────────────────────────────────────────────
    send_event(window, 0, 0, State::Done);
    Ok(())
}

/// Main patch entrypoint—replaces the old etag‐based flow.
pub async fn patch(
    window: Window,
    client: reqwest::Client,
    patcher_url: String,
    patcher_resp: PatcherResponse,
    game_folder: PathBuf,
    cancel: CancellationToken,
) {
    // ─── Roll back any different server that might still be active ───────────
    let active_file = game_folder.join(ACTIVE_SERVER_FILE);
    let prev_server = fs::read_to_string(&active_file)
        .unwrap_or_default()
        .trim()
        .to_string();

    if prev_server != patcher_resp.server_name && !prev_server.is_empty() {
        if let Err(e) = restore_server(&game_folder, &prev_server) {
            warn!("failed to restore {prev_server}: {e}");
        }
    }
	// 1) version gate
    let local_version = fs::read_to_string(CLIENT_VERSION_FILE)
        .unwrap_or_default()
        .trim()
        .to_string();

    // fetch ButterVersion.txt
    let server_version = match client
        .get(format!("{}/ButterVersion.txt", &patcher_url))
        .send()
        .await
    {
        Ok(resp) => resp
            .text()
            .await
            .unwrap_or_default()
            .trim()
            .to_string(),
        Err(_) => "".to_string(),
    };

    // If versions match, skip patch entirely
    if server_version == local_version {
        // remember which server is active even though no patch ran
        if let Some(dir) = active_file.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = fs::write(&active_file, &patcher_resp.server_name);

        send_event(&window, 0, 0, State::Done);
        return;
    }

    // 2) proceed with the normal patch flow
    let tmp_folder = game_folder.join("tmp");
    if let Err(e) = fs::create_dir_all(&tmp_folder) {
        warn!("error creating patcher dir: {}", e);
        send_error(&window, FILE_ERROR);
        return;
    }

    patch_internal(
        &window,
        client,
        patcher_url,
        patcher_resp.clone(),
        &game_folder,
        &tmp_folder,
        cancel,
    )
    .await
    .unwrap_or_else(|e| send_error(&window, e));

    if let Err(e) = fs::remove_dir_all(&tmp_folder) {
        warn!("error deleting patcher dir: {}", e);
        send_error(&window, FILE_ERROR);
    }
	
    // record the server just patched
    if let Some(dir) = active_file.parent() {
        let _ = fs::create_dir_all(dir);
    }
    if let Err(e) = fs::write(&active_file, &patcher_resp.server_name) {
        warn!("failed to write active-server file: {}", e);
    }

    // 3) write the new version so next launch is up-to-date
    if let Err(e) = fs::write(CLIENT_VERSION_FILE, server_version) {
        warn!("failed to write {}: {}", CLIENT_VERSION_FILE, e);
    }
}

#[tauri::command]
pub async fn reset_game_files(game_folder: String) -> Result<(), String> {
    use std::path::PathBuf;

    // same constant
    const ACTIVE_SERVER_FILE: &str = "ButterClient/active_server";

    let root        = PathBuf::from(&game_folder);
    let active_file = root.join(ACTIVE_SERVER_FILE);
    let server      = std::fs::read_to_string(&active_file)
        .unwrap_or_default()
        .trim()
        .to_string();

    // nothing to do if player never patched
    if server.is_empty() {
        return Ok(());
    }

    // roll back everything this server touched
    restore_server(&root, &server)
        .map_err(|e| format!("restore failed: {e}"))?;

    // remove the “active server” marker so next launch is pristine
    let _ = std::fs::remove_file(active_file);
    Ok(())
}
