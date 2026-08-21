// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_fs::FsExt;

/// Cold-start handoff for a file the OS handed us (Finder open / CLI argv)
/// before the webview existed.
///
/// `frontend_ready` flips the first time the frontend drains the slot. After
/// that the webview is listening, so deliveries ride the `open-file` event and
/// the slot is deliberately left empty — otherwise a webview reload would
/// replay a stale path and discard in-progress edits.
#[derive(Default)]
struct PendingState {
    path: Option<String>,
    frontend_ready: bool,
}

#[derive(Default)]
struct PendingFile(Mutex<PendingState>);

/// Hand `path` to the frontend by whichever route can currently reach it.
///
/// Always emits `open-file` (heard by a warm webview) and, only before the
/// frontend's first drain, parks the path in the pending slot (read by a cold
/// webview once it comes up). Both are done under one lock so a delivery racing
/// the drain cannot leave a stale path behind.
fn deliver_file(app: &tauri::AppHandle, path: String) {
    // The capability scope's glob cannot match a dot-prefixed path component
    // (`require_literal_leading_dot` is true on unix), so `~/.claude/NOTES.md`
    // would be refused. Grant this exact file at runtime — an escaped literal
    // pattern, the same mechanism the dialog and drag-drop plugins use. Runtime
    // grants are ORed with the capability scope.
    let _ = app.fs_scope().allow_file(&path);

    {
        let state = app.state::<PendingFile>();
        let mut pending = state.0.lock().unwrap_or_else(|e| e.into_inner());
        if !pending.frontend_ready {
            pending.path = Some(path.clone());
        }
    }

    let _ = app.emit("open-file", path);
}

/// Returns the pending file path and clears it, so a file is only consumed once.
/// Also marks the frontend as ready — see `PendingState::frontend_ready`.
#[tauri::command]
fn get_pending_file(state: tauri::State<'_, PendingFile>) -> Option<String> {
    let mut pending = state.0.lock().unwrap_or_else(|e| e.into_inner());
    pending.frontend_ready = true;
    pending.path.take()
}

/// First non-flag CLI argument, canonicalized to an absolute path.
/// macOS adds a `-psn_0_...` argument on Finder launches, hence the flag skip.
/// Uses `args_os` because `args` panics on non-UTF-8 arguments and the release
/// profile is `panic = "abort"`.
fn file_from_args() -> Option<String> {
    std::env::args_os()
        .skip(1)
        .find(|arg| !arg.to_string_lossy().starts_with('-'))
        .and_then(|arg| std::fs::canonicalize(arg).ok())
        .map(|path| path.to_string_lossy().into_owned())
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(PendingFile::default())
        .invoke_handler(tauri::generate_handler![get_pending_file])
        .setup(|app| {
            if let Some(path) = file_from_args() {
                deliver_file(app.handle(), path);
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building markdiff");

    app.run(|_app_handle, _event| {
        // macOS delivers Finder / `open` requests as Apple Events, never argv.
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if let tauri::RunEvent::Opened { urls } = _event {
            // Open the first file only, matching the drag-drop handler's paths[0].
            if let Some(url) = urls.first() {
                // to_file_path() undoes percent-encoding, so paths with spaces
                // round-trip correctly. String stripping would not.
                if let Ok(path) = url.to_file_path() {
                    deliver_file(_app_handle, path.to_string_lossy().into_owned());
                }
            }
        }
    });
}
