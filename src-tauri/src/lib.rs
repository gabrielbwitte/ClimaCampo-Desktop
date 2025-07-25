mod api;
mod models;
mod calls_ipc;

use calls_ipc::calls::{login_ipc, logoff_ipc};

use crate::api::user::ApiReq;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ApiReq::new("https://www.farmnext.com.br"))
        .invoke_handler(tauri::generate_handler![
            login_ipc,
            logoff_ipc
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
