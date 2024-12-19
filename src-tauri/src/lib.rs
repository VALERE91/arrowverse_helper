use cache::prepare_cache;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod cache;
mod db;
mod models;
mod schema;
mod tmdb;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|_app| {
            db::init();
            tauri::async_runtime::spawn(async move { prepare_cache().await });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
