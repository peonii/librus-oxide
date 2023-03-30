use std::sync::Arc;

use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::api_types::APISynergiaAccountsWrapper;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

pub struct Librus {
    request: Client,
    bearer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrusCredentials {
    pub email: String,
    pub password: String,
}

impl Default for Librus {
    fn default() -> Self {
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::default();
        let cookie_store = Arc::new(cookie_store);

        Self {
            request: Client::builder()
                .user_agent(USER_AGENT)
                .cookie_store(true)
                .cookie_provider(Arc::clone(&cookie_store))
                .build()
                .unwrap(),
            bearer: String::new(),
        }
    }
}

impl Librus {
    async fn get_csrf(&self) -> Result<String> {
        let response = self.request.get("https://portal.librus.pl/").send().await?;

        // Find csrf token

        let response_text = response.text().await?;

        // Search for regex: /<meta name="csrf-token" content="(.*)">/g
        let re = regex::Regex::new(r#"<meta name="csrf-token" content="(.*)">"#)?;

        let csrf = re
            .captures(&response_text)
            .ok_or(anyhow::anyhow!("Couldn't fetch the CSRF token!"))?
            .get(1)
            .ok_or(anyhow::anyhow!("Couldn't fetch the CSRF token!"))?
            .as_str();

        Ok(csrf.to_string())
    }

    pub async fn login(&mut self, credentials: &LibrusCredentials) -> Result<()> {
        let mut headers = HeaderMap::new();
        let csrf = self.get_csrf().await?;

        headers.insert("X-CSRF-TOKEN", csrf.parse()?);
        headers.insert("User-Agent", USER_AGENT.parse()?);
        headers.insert("Content-Type", "application/json".parse()?);

        let response_cookies = self
            .request
            .post("https://portal.librus.pl/konto-librus/login/action")
            .headers(headers)
            .json(credentials)
            .send()
            .await?;

        // Check for correct response
        if response_cookies.status() != 200 {
            return Err(anyhow::anyhow!(
                "Got an invalid response while fetching cookies!\nInvalid email or password\n{:?}",
                response_cookies
            ));
        }

        let response = self
            .request
            .get("https://portal.librus.pl/api/v3/SynergiaAccounts")
            .send()
            .await?;

        // Find bearer token
        let accounts = match response.json::<APISynergiaAccountsWrapper>().await {
            Ok(accounts) => accounts,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Got an invalid response while fetching the access token!\nInvalid email or password"
                ))
            }
        };

        if accounts.accounts.is_empty() {
            return Err(anyhow::anyhow!(
                "Got an invalid response while fetching the access token!\nInvalid email or password"
            ));
        }

        self.bearer = accounts.accounts[0].access_token.clone();

        Ok(())
    }

    pub async fn request<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            ("Bearer ".to_owned() + &self.bearer).parse()?,
        );

        headers.insert("User-Agent", USER_AGENT.parse()?);
        headers.insert("gzip", "true".parse()?);

        let req = self.request.get(url).headers(headers).send().await?;

        let response = req.json::<T>().await?;

        Ok(response)
    }
}
