use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct ObjDeleteSession {
    #[serde(rename = "deletedCount")]
    pub deleted_count: u16
    }

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectId {
    #[serde(rename = "$oid", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    #[serde(rename = "$oid")]
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectProfileUser {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    pub name: String,
    pub email: String,
    pub access: Access
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Access {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_d_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub climate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapa: Option<bool>,
}