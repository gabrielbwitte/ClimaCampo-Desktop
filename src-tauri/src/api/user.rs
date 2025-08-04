use std::{collections::HashMap, sync::{Arc, Mutex}};

use reqwest::cookie::{CookieStore, Jar};
use tauri::{Url};
use tauri_plugin_http::reqwest;

use crate::models::user::{ObjDeleteSession, ObjectId, ObjectProfileUser};

#[derive(Debug)]
pub struct ApiReq {
    client: reqwest::Client,
    url: String,
    cookie_store: Arc<Jar>,
    id_user: Arc<Mutex<String>>
}

impl ApiReq {
    pub fn new(base_url: &str) -> Arc<Self> {
        let cookie_store = Arc::new(Jar::default());
        let id_user = Arc::new(Mutex::new("".to_string()));
        
        let client = reqwest::Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()
            .unwrap();

        Arc::new(Self {
            client: client, 
            url: base_url.to_string(), 
            cookie_store,
            id_user
            }
        )
    }

    pub async fn login(&self, username: String, password: String) -> u16 {
        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);

        let res = self.client.post(format!("{}/login", self.url))
            .json(&map)
            .send()
            .await;

        match res {
            Ok(response) => {
                let header = response.status();
                let json = match response.json::<ObjectId>().await {
                    Ok(v) => v,
                    Err(_) => ObjectId { id: None }
                };
                if let Some(v) = json.id {
                    println!("{}", v);
                    let mut id_user = self.id_user.lock().unwrap();
                    id_user.clear();
                    id_user.push_str(&v); 
                }
                
                header.as_u16()
            },
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

        match res {
            Ok(response) => {
                println!("Arc =>: {:?}", self.id_user.lock().unwrap());
                return response.deleted_count;
            },
            Err(_) => 500
        }
    }

    pub async fn get_user_profile(&self) -> Result<ObjectProfileUser, String> {
        let parsed_url = Url::parse(&self.url).unwrap();
        let cookie_header = self.cookie_store.cookies(&parsed_url);

        let id_user = match self.id_user.lock() {
            Ok(v) => v.to_string(),
            Err(_) => return Err("Error".to_string())
        };


        let req = self.client.get(format!("{}/user/profile/{}", self.url, id_user));

        let req = if let Some(cookie_str) = cookie_header {
            let token_value = cookie_str.to_str().unwrap()
                .split('=')
                .nth(1)
                .expect("Formato de token invalido");
            req.header("token", token_value)
        } else {
            req
        };

        let res = req.send().await;

        match res {
            Ok(result) => {
                match result.json::<ObjectProfileUser>().await {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        println!("Err =>: {:?}", e);
                        Err("Erro".to_string())
                    }
                }
            }
            Err(e) => {
                println!("Req =>: {:?}", e);
                Err("Erro".to_string())
            }
        } 
    }
}