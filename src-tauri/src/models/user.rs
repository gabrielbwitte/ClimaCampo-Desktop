use serde::{Deserialize};
#[derive(Debug, Deserialize)]
pub struct ObjDeleteSession {
    pub deletedCount: u16
    }