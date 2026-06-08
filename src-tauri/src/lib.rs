mod error;
pub use error::{LauncherError, Result};
mod commands;
mod business;
mod infra;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![crate::commands::config::get_config])
        .invoke_handler(tauri::generate_handler![crate::commands::auth::login])
        .invoke_handler(tauri::generate_handler![crate::commands::auth::logout])
        .invoke_handler(tauri::generate_handler![crate::commands::auth::current_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
