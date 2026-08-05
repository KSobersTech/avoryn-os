mod domain;
mod infrastructure;

use infrastructure::database::initialize_database;
use tauri::Manager;

// Learn more about Tauri commands at:
// https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let database_pool = tauri::async_runtime::block_on(initialize_database(app.handle()))?;

            app.manage(database_pool);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running AVORYN");
}
