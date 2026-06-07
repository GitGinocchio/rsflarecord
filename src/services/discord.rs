use std::sync::{Arc, OnceLock};

use reqwest::{Client, header::{self, HeaderValue}};
use twilight_model::{
    id::{Id, marker::UserMarker}, 
    user::User as TwilightUser
};

use crate::{error::Error, models::user::User};



pub (crate) static DISCORD_SERVICE: OnceLock<Arc<DiscordService>> = OnceLock::new();

const BASE_URL: &str = concat!("https://discord.com/api/v", "10");

#[allow(unused)]
pub struct DiscordService {
    client: Client,
    token: String
}

#[allow(unused)]
impl DiscordService {
    pub fn get_or_init(client: Client, token: String) -> Arc<DiscordService> {
        DISCORD_SERVICE.get_or_init(|| {
            let service = DiscordService::new(client, token);
            Arc::new(service)
        }).clone()
    }

    pub (crate) fn new(client: Client, token: String) -> Self {
        Self {
            client,
            token
        }
    }

    pub (crate) fn build_client(token: &str) -> Result<Client, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bot {}", token))?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Client::builder()
            .default_headers(headers)
            .build()
            .map_err(Error::UpstreamError)
    }

    pub async fn fetch_user(&self, user_id: &Id<UserMarker>) -> Result<User, Error> {
        let endpoint = format!("{}/users/{}", BASE_URL, user_id);

        let user: TwilightUser = self.client.get(endpoint)
            .send()
            .await?
            .json()
            .await?;

        Ok(User::from(user))
    }
}