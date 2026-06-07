use std::sync::Arc;
use worker::Env;

use crate::{
    bot::Bot, 
    models::autocomplete::data::AutocompleteData, 
    services::discord::DiscordService
};



pub struct AutocompleteContext {
    pub bot: Arc<Bot>,
    pub env: Env,
    pub data: AutocompleteData,

    pub discord: Arc<DiscordService>
}

impl AutocompleteContext {
    pub fn new(bot: Arc<Bot>, env: Env, data: AutocompleteData, discord: Arc<DiscordService>) -> Self {
        Self {
            bot: bot, 
            env: env,
            data: data,
            discord: discord
        }
    }

    pub (crate) fn with_data(self, inner_data: AutocompleteData) -> Self {
        Self::new(self.bot, self.env, inner_data, self.discord)
    }
}