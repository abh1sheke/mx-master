// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod excel;
pub mod dns;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![excel::save_to, dns::query_batcher])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
