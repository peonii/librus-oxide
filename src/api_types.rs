use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct APISynergiaAccountsWrapper {
    pub accounts: Vec<APISynergiaAccount>,
}

#[derive(Serialize, Deserialize)]
pub struct APISynergiaAccount {
    pub id: i32,
    pub accessToken: String,
    pub login: String,
}
