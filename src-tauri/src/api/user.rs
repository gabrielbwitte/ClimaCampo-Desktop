use std::collections::HashMap;
use tauri_plugin_http::reqwest;
use crate::models::user::{ObjJson};

#[tauri::command(async)]
pub async fn login(user: String, password: String) -> u16 {
    let mut map = HashMap::new();
    map.insert("username", user);
    map.insert("password", password);

    let client = reqwest::Client::new();
    let res = client.post("https://www.farmnext.com.br/login")
        .json(&map)
        .send()
        .await;

    match res {
        Ok(value_body) => {
            let vstatus = value_body.status().as_u16();
            let json_content = match value_body.json::<ObjJson>().await {
                Ok(v) => (v.data, v.token),
                Err(_) => return vstatus
            };
            println!("{}: {}", json_content.0, json_content.1);
            return vstatus
        },
        Err(_) => return 502
    };
}