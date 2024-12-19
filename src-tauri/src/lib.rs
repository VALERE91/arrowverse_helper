use std::sync::Mutex;

use cache::Cache;
use tauri::Manager;

mod cache;
mod db;
mod handlers;
mod models;
mod schema;
mod tmdb;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> anyhow::Result<()> {
    let cache = Cache::new();
    tauri::Builder::default()
        .setup(|app| {
            db::init();
            app.manage(Mutex::new(cache));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handlers::get_shows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
