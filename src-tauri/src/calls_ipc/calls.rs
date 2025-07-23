use tauri::{AppHandle, Emitter};

use crate::api::user::ApiReq;


#[tauri::command]
pub async fn login_ipc(username: String, password: String) -> u16 {
    let api = ApiReq::new("https://www.farmnext.com.br");

    let status = api.login(username, password).await;

    api.logoff().await;

    status
}

#[tauri::command]
pub async fn logoff_ipc(app: AppHandle) {
    println!("logoff...");
    let api = ApiReq::new("https://www.farmnext.com.br");

    api.logoff().await;
    
    app.emit("logoff_command_ipc", 401).unwrap();
}
