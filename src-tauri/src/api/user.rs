use std::{collections::HashMap, sync::Arc};

use reqwest::cookie::{CookieStore, Jar};
use tauri::Url;
use tauri_plugin_http::reqwest;

use crate::models::user::ObjDeleteSession;

#[derive(Debug)]
pub struct ApiReq {
    client: reqwest::Client,
    url: String,
    cookie_store: Arc<Jar>
}

impl ApiReq {
    pub fn new(base_url: &str) -> Self {
        let cookie_store = Arc::new(Jar::default());

        let client = reqwest::Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()
            .unwrap();

        Self { client: client, url: base_url.to_string(), cookie_store, }
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

    pub async fn logoff(&self) {
        let url = format!("{}/logoff", self.url);
        let parsed_url = Url::parse(&self.url).unwrap();
        let cookie_header = self.cookie_store.cookies(&parsed_url);

        let req = self.client.delete(url);

        let req = if let Some(cookie_str) = cookie_header {
            req.header("token", cookie_str.to_str().unwrap())
        } else {
            req
        };

    
        let res = req.send().await.unwrap().json::<ObjDeleteSession>().await;

        println!("{:?}", res);

        match res {
            Ok(response) => {
                println!("{:?}", response);
            },
            Err(error) => println!("{:?}", error)
        }
    }
}