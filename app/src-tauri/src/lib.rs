mod preview;
// mod watch;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(preview::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            preview::start_preview,
            preview::stop_preview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
