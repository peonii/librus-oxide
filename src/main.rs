use librus::client::Librus;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct MeResponse {
    Me: LibrusMe,
}

#[derive(Deserialize, Serialize, Debug)]
struct LibrusAccount {
    Id: i32,
    UserId: i32,
    FirstName: String,
    LastName: String,
    Email: String,
    Login: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LibrusMe {
    Account: LibrusAccount,
}

#[tokio::main]
async fn main() {
    let mut librus = Librus::default();

    let credentials = librus::client::LibrusCredentials {
        email: "".to_string(),
        password: "".to_string(),
    };

    librus.login(&credentials).await.unwrap();

    let account: MeResponse = librus
        .request::<MeResponse>("https://api.librus.pl/3.0/Me")
        .await
        .unwrap();

    println!("{:?}", account);
}
