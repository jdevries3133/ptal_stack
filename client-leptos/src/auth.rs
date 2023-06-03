use anyhow::Result;
use reqwest::{Client};
use serde::{Deserialize, Serialize};

use common::models::{LoginPayload, LoginResponse};

use super::api::get_url;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Auth {
    token: String,
}

impl Auth {
    pub fn new(token: &str) -> Self {
        Auth {
            token: token.to_string(),
        }
    }
    pub async fn from_credentials(credentials: &LoginPayload) -> Result<Auth> {
        let c = Client::new();
        let res: LoginResponse = c
            .post(get_url("/api/v1/auth/login"))
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .json(credentials)
            .send()
            .await?
            .json()
            .await?;

        Ok(Auth::new(&res.session_token))
    }
}
