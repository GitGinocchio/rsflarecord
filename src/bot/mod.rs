use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use twilight_model::application::interaction::Interaction as TwilightInteraction;
use worker::{Env, Request, Response};

use crate::bot::builder::BotBuilder;
use crate::models::command::context::CommandContext;
use crate::models::command::{CommandHandler, CommandResult, CommandType};
use crate::models::components::ComponentType;
use crate::models::interaction::Interaction;
use crate::models::modal::ModalType;
use crate::error::Error;
use crate::crypto;

pub mod builder;

pub static BOT: OnceLock<Arc<RwLock<Bot>>> = OnceLock::new();

#[allow(unused)]
pub struct Bot {
    pub (crate) commands: HashMap<String, CommandType>,
    pub (crate) components: HashMap<String, ComponentType>,
    pub (crate) modals: HashMap<String, ModalType>
}

#[allow(unused)]
impl Bot {
    pub (crate) fn set_global(self) {
        let bot = Arc::new(RwLock::new(self));
        BOT.set(bot).expect_err("Bot already initialized");
    }

    pub (crate) fn get_global() -> Arc<RwLock<Bot>> {
        BOT.get().expect("Bot not initiliazed").clone()
    }

    pub fn new() -> Arc<RwLock<Bot>> {
        let bot = Self {
            commands: HashMap::new(),
            components: HashMap::new(),
            modals: HashMap::new()
        };
        bot.set_global();
        Bot::get_global()
    }

    pub fn register_command_handler<F, Fut>(&mut self, 
        name: impl Into<String>, 
        description: impl Into<String>, 
        handler: F
    )
    where 
        F: Fn(Interaction, CommandContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CommandResult> + Send + Sync + 'static,
    {
        let handler = CommandHandler::new(name.into(), description.into(), handler);

        let bot = Bot::get_global();
        let mut bot_guard = bot.write().expect("Lock poisoned");
        bot_guard.commands.insert(handler.name.clone(), Box::new(handler));
    }

    pub async fn handle(&self, mut req: Request, env: Env) -> worker::Result<Response> {
        let body = req.bytes().await?;
        let headers = req.headers();

        let public_key = env.secret("DISCORD_PUBLIC_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();
    
        let is_valid = crypto::verify_signature(headers, &body, &public_key)?;

        if !is_valid {
            return Response::error("Unauthorized", 401);
        }

        let tw_interaction: TwilightInteraction = serde_json::from_slice(&body)?;
        let interaction = Interaction::from(tw_interaction);

        match interaction.perform(env).await {
            Ok(response) => Ok(response),
            Err(e) => e.as_response()
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