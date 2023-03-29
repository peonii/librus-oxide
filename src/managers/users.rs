use crate::client::Librus;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct MeResponse {
    #[serde(alias = "Me")]
    me: LibrusMe,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LibrusAccount {
    #[serde(alias = "Id")]
    id: i32,

    #[serde(alias = "UserId")]
    user_id: i32,

    #[serde(alias = "FirstName")]
    first_name: String,

    #[serde(alias = "LastName")]
    last_name: String,

    #[serde(alias = "Email")]
    email: String,

    #[serde(alias = "Login")]
    login: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LibrusMe {
    #[serde(alias = "Account")]
    account: LibrusAccount,
}

impl Librus {
    pub async fn me(&self) -> Result<LibrusAccount> {
        let me_response = self
            .request::<MeResponse>("https://api.librus.pl/3.0/Me")
            .await?;

        Ok(me_response.me.account)
    }
}