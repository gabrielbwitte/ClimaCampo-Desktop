use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct ObjJson {
    pub data: String,
    pub token: String
}