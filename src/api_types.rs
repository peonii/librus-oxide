use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct APISynergiaAccountsWrapper {
    pub accounts: Vec<APISynergiaAccount>,
}

#[derive(Serialize, Deserialize)]
pub struct APISynergiaAccount {
    pub id: i32,
    #[serde(alias = "accessToken")]
    pub access_token: String,
    pub login: String,
}
