use std::sync::Arc;
use worker::Env;

use crate::{
    bot::Bot, 
    services::discord::DiscordService
};

pub struct ComponentContext {
    pub bot: Arc<Bot>,
    pub env: Env,
    pub discord: Arc<DiscordService>
}

impl ComponentContext {
    pub fn new(bot: Arc<Bot>, env: Env, discord: Arc<DiscordService>) -> Self {
        Self {
            bot: bot, 
            env: env,
            discord: discord
        }
    }
}