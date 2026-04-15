// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Emitter;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let args: Vec<String> = std::env::args().skip(1).collect();
            if let Some(path) = args.first() {
                let path = path.clone();
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    let _ = handle.emit("open-file", path);
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running markdiff");
}
