// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod util;

use file::FileHandler;
use tauri::State;
use util::{StrResult, StringifyResult};

fn main() {
    let specta = tauri_specta::ts::builder().commands(tauri_specta::collect_commands![connect]);

    #[cfg(debug_assertions)]
    let specta = specta.path("../src/lib/specta.ts");

    tauri::Builder::default()
        .manage(FileHandler::new())
        .plugin(specta.into_plugin())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
async fn connect(file: State<'_, FileHandler>) -> StrResult<()> {
    file.connect("../example.db", true).await.stringify()?;
    return Ok(());
}
