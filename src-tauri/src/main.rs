// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod dns;
pub mod excel;

#[tauri::command]
fn sig() {
    panic!("initiated signal!");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![excel::save_to, dns::query_batcher, sig])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
