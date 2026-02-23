// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod util;
mod downloads;

use downloads::facade::{
    Facade,
    start_download,
    get_download_info,
    early_download_cancel,
};


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

///# Panics
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Facade::new())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![start_download, get_download_info, early_download_cancel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
