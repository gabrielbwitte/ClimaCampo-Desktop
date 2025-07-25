use std::{collections::HashMap, sync::Arc};

use reqwest::cookie::{CookieStore, Jar};
use tauri::{Url};
use tauri_plugin_http::reqwest;

use crate::models::user::ObjDeleteSession;

#[derive(Debug)]
pub struct ApiReq {
    client: reqwest::Client,
    url: String,
    cookie_store: Arc<Jar>
}

impl ApiReq {
    pub fn new(base_url: &str) -> Arc<Self> {
        let cookie_store = Arc::new(Jar::default());

        let client = reqwest::Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()
            .unwrap();

        Arc::new(Self {
             client: client, 
             url: base_url.to_string(), 
             cookie_store 
            }
        )
    }

    pub async fn login(&self ,username: String, password: String) -> u16 {
        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);

        let res = self.client.post(format!("{}/login", self.url))
            .json(&map)
            .send()
            .await;

        match res {
            Ok(response) => response.status().as_u16(),
            Err(_) => 500
        }
    }

    pub async fn logoff(&self) -> u16 {
        let parsed_url = Url::parse(&self.url).unwrap();
        let cookie_header = self.cookie_store.cookies(&parsed_url);

        
        let req = self.client.delete(format!("{}/logoff", self.url));

        let req = if let Some(cookie_str) = cookie_header {
            let token_value = cookie_str.to_str().unwrap()
                .split('=')
                .nth(1)
                .expect("Formato de token invalido");
            req.header("token", token_value)
        } else {
            req
        };

        let res = req.send().await.unwrap().json::<ObjDeleteSession>().await;

        println!("{:?}", &res);

        match res {
            Ok(response) => {
                return response.deletedCount;
            },
            Err(_) => 500
        }
    }
}