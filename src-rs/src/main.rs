// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod model;
mod util;

use file::FileHandler;
use tauri::{Manager, State};
use util::{StrResult, StringifyResult};

fn main() {
    tauri::Builder::default()
        .manage(FileHandler::new())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window_shadows::set_shadow(&window, true).expect("Unsupported platform.");
            return Ok(());
        })
        .invoke_handler(bind_commands![
            connect,
            disconnect,
            is_connected,
            get_timelines
        ])
        .run(tauri::generate_context!())
        .expect("Error trying to run tauri application");
}

#[tauri::command]
#[specta::specta]
async fn connect(file: State<'_, FileHandler>, path: &str, create: bool) -> StrResult<()> {
    file.connect(path, create).await.stringify()?;
    return Ok(());
}

#[tauri::command]
#[specta::specta]
async fn disconnect(file: State<'_, FileHandler>) -> StrResult<()> {
    file.disconnect().await;
    return Ok(());
}

#[tauri::command]
#[specta::specta]
async fn is_connected(file: State<'_, FileHandler>) -> StrResult<bool> {
    return Ok(file.is_connected().await);
}

#[tauri::command]
#[specta::specta]
async fn get_timelines(file: State<'_, FileHandler>) -> StrResult<Vec<model::Timeline>> {
    return file.get_timelines().await.stringify();
}
