// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
use tauri::Manager;


use commands::{
    contract_status::{get_contract_status, start_contract_status_watcher, stop_contract_status_watcher, StatusWatcherHandle},
    process::{start_process, kill_process}
};


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(StatusWatcherHandle::default())
        .setup(|app| {
            app.get_webview_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_contract_status,
            start_contract_status_watcher,
            stop_contract_status_watcher,
            start_process,
            kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}