// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_dev_tools::diff_text;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn compare(content1: &str, content2: &str) -> Option<(String, String)> {
    diff_text(content1, content2).ok()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, compare])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
