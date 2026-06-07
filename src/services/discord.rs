use std::sync::{Arc, OnceLock};

use reqwest::Client;
use twilight_model::{
    id::{Id, marker::UserMarker}, 
    user::User as TwilightUser
};

use crate::{error::Error, models::user::User};



pub (crate) static DISCORD_SERVICE: OnceLock<Arc<DiscordService>> = OnceLock::new();

const BASE_URL: &str = concat!("https://discord.com/api/v", "10");

#[allow(unused)]
pub struct DiscordService {
    client: Arc<Client>
}

#[allow(unused)]
impl DiscordService {
    pub (crate) fn get_or_init(client: Arc<Client>) -> Arc<DiscordService> {
        DISCORD_SERVICE.get_or_init(|| {
            let service = DiscordService::new(client);
            Arc::new(service)
        }).clone()
    }

    pub (crate) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub (crate) async fn update_global_commands(
        &self, 
        application_id: &str, 
        commands: &str
    ) -> Result<(), Error> {
        let url = format!("{}/applications/{}/commands", BASE_URL, application_id);

        self.client.put(url)
            .body(commands.to_string())
            .send()
            .await
            .map_err(Error::ReqwestError)?
            .error_for_status()
            .map_err(Error::ReqwestError)?;

        Ok(())
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