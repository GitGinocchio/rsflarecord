use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};
use twilight_model::application::interaction::Interaction as TwilightInteraction;
use worker::{Env, Request, Response};

use crate::bot::builder::BotBuilder;
use crate::models::command::serializable::SerializableCommand;
use crate::models::command::{CommandType};
use crate::models::components::ComponentType;
use crate::models::interaction::Interaction;
use crate::models::modal::ModalType;
use crate::error::Error;
use crate::crypto;

pub mod builder;

pub (crate) static HTTP_CLIENT: OnceLock<Arc<Client>> = OnceLock::new();
static BOT: OnceLock<Arc<Bot>> = OnceLock::new();
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[allow(unused)]
pub struct Bot {
    pub (crate) commands: HashMap<String, CommandType>,
    pub (crate) components: HashMap<String, ComponentType>,
    pub (crate) modals: HashMap<String, ModalType>
}

#[allow(unused)]
impl Bot {
    pub (crate) fn set_global(self) {
        let bot = Arc::new(self);
        BOT.set(bot).map_err(|_| worker::console_debug!("Bot already initialized"));
    }

    pub (crate) fn get_global() -> Arc<Bot> {
        BOT.get().expect("Bot not initiliazed").clone()
    }

    pub (crate) fn init_client(&self, token: &str) -> Client {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bot {}", token)).expect("Error parsing header value"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Client::builder()
            .default_headers(headers)
            .build()
            .expect("Error building reqwest::Client")
    }

    pub (crate) fn new() -> Arc<Bot> {
        let bot = Self {
            commands: HashMap::new(),
            components: HashMap::new(),
            modals: HashMap::new()
        };
        bot.set_global();
        Bot::get_global()
    }

    pub async fn sync_commands_once(&self, env: &Env) -> worker::Result<bool> {
        if IS_INITIALIZED.load(Ordering::Acquire) {
            worker::console_debug!("Command synchronization not necessary");
            return Ok(true);
        }

        worker::console_debug!("Launching command synchronization");

        let application_id = env.secret("DISCORD_BOT_APPLICATION_ID")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        let token = env.secret("DISCORD_BOT_TOKEN")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        if HTTP_CLIENT.get().is_none() {
            let token = env.secret("DISCORD_BOT_TOKEN")
                .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
                .to_string();

            let client = self.init_client(&token);
            HTTP_CLIENT.set(Arc::new(client));
        }

        let client = HTTP_CLIENT.get().expect("HTTP_CLIENT not initialized!");

        let url = format!(
            "https://discord.com/api/v10/applications/{}/commands",
            application_id
        );

        let serializable_commands: Vec<SerializableCommand<'_>> = self.commands.values()
            .map(|cmd| SerializableCommand(cmd))
            .collect();

        let serialized_commands = serde_json::to_string(&serializable_commands).map_err(|e| Error::JsonFailed(e))?;
        worker::console_log!("Sending  : {}", serialized_commands);

        client
            .put(url)
            .header("Authorization", format!("Bot {}", token))
            .header("Content-Type", "application/json")
            .body(serialized_commands)
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e))?;

        IS_INITIALIZED.store(true, Ordering::Release);

        Ok(false)
    }

    pub async fn handle(&self, mut req: Request, env: Env) -> worker::Result<Response> {
        let body = req.bytes().await?;
        let headers = req.headers();

        let public_key = env.secret("DISCORD_BOT_PUBLIC_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        if HTTP_CLIENT.get().is_none() {
            let token = env.secret("DISCORD_BOT_TOKEN")
                .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
                .to_string();

            let client = self.init_client(&token);
            HTTP_CLIENT.set(Arc::new(client));
        }
    
        let is_valid = crypto::verify_signature(headers, &body, &public_key)?;

        if !is_valid {
            return Response::error("Unauthorized", 401);
        }

        let tw_interaction: TwilightInteraction = serde_json::from_slice(&body)?;
        let interaction = Interaction::from(tw_interaction);

        match interaction.perform(env).await {
            Ok(response) => Ok(response),
            Err(e) => {
                worker::console_debug!("Handler error: {e:?}");
                e.as_response()
            }
        }
    }
}

impl From<BotBuilder> for Bot {
    fn from(builder: BotBuilder) -> Self {
        Self {
            commands: builder.commands,
            components: builder.components,
            modals: builder.modals
        }
    }
}