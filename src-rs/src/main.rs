// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod model;
mod util;

use tauri::Manager;
use util::{StrResult, StringifyResult};

#[macro_export]
macro_rules! bind_commands {
    [ $($ident:ident),+ ] => {
        {
            // Create or update TypeScript bindings when debugging
            #[cfg(debug_assertions)]
            tauri_specta::ts::export(
                specta::collect_types![$($ident),+],
                "../src/lib/bindings.ts",
            ).expect("Failed to export types");

            // Create the Tauri plugin to register commands
            tauri::generate_handler![$($ident),+]
        }
    };
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window_shadows::set_shadow(&window, true).expect("Unsupported platform.");
            return Ok(());
        })
        .invoke_handler(bind_commands![
            connect,
            disconnect,
            is_connected,
            get_timelines,
            get_events
        ])
        .run(tauri::generate_context!())
        .expect("Error trying to run tauri application");
}

#[tauri::command]
#[specta::specta]
async fn connect(path: &str, create: bool) -> StrResult<()> {
    file::connect(path, create).await.stringify()?;
    return Ok(());
}

#[tauri::command]
#[specta::specta]
async fn disconnect() -> StrResult<()> {
    file::disconnect().await;
    return Ok(());
}

#[tauri::command]
#[specta::specta]
async fn is_connected() -> StrResult<bool> {
    return Ok(file::is_connected().await);
}

#[tauri::command]
#[specta::specta]
async fn get_timelines() -> StrResult<Vec<model::Timeline>> {
    return file::get_timelines().await.stringify();
}

#[tauri::command]
#[specta::specta]
async fn get_events() -> StrResult<Vec<model::Event>> {
    return file::get_events().await.stringify();
}
