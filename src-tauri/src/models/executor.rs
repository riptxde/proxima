use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub id: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct ExecuteRequest {
    pub client_ids: Vec<String>,
    pub script: String,
}
