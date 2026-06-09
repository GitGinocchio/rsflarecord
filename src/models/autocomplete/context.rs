use std::sync::Arc;
use worker::Env;

use crate::{
    bot::{state::BotState}, 
    services::discord::DiscordService
};



pub struct AutocompleteContext {
    pub bot: BotState,
    pub env: Env,
    pub discord: Arc<DiscordService>
}

impl AutocompleteContext {
    pub fn new(bot: BotState, env: Env, discord: Arc<DiscordService>) -> Self {
        Self {
            bot: bot, 
            env: env,
            discord: discord
        }
    }
}