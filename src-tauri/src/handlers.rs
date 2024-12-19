use std::sync::Mutex;

use tauri::State;

use crate::{cache::Cache, models::show::TVShow};

#[tauri::command]
pub fn get_shows(state: State<'_, Mutex<Cache>>) -> Vec<TVShow> {
    state.lock().unwrap().get_all_shows().unwrap()
}
