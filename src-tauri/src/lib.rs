mod api;
mod models;
mod calls_ipc;

use calls_ipc::calls::{login_ipc, logoff_ipc};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            login_ipc,
            logoff_ipc
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
