use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,

    #[serde(rename = "password")]
    pub hashed_password: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Token {
    pub token: String,
    pub user_id: String,
    pub expire_until: u64,
}
