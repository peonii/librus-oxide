use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NumberIDResource {
    #[serde(alias = "Id")]
    pub id: i32,

    #[serde(alias = "Url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringIDResource {
    #[serde(alias = "Id")]
    pub id: String,

    #[serde(alias = "Url")]
    pub url: String,
}