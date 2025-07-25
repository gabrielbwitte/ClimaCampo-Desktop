use std::sync::Arc;

use tauri::{AppHandle, Emitter, State};

use crate::api::user::ApiReq;

#[tauri::command]
pub async fn login_ipc(state: State<'_, Arc<ApiReq>> ,username: String, password: String) -> Result<u16, String> {
    let status = state.login(username, password).await;
    Ok(status)
}

#[tauri::command]
pub async fn logoff_ipc(state: State<'_, Arc<ApiReq>>, app: AppHandle) -> Result<(), ()> {
    let res = state.logoff().await;
    app.emit("logoff_command_ipc", res).expect("500");
    Ok(())
}
