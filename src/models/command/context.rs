use std::sync::Arc;
use worker::Env;

use crate::{
    bot::Bot, 
    models::command::{
        data::CommandData
    }
};



pub struct CommandContext {
    pub bot: Arc<Bot>,
    pub env: Env,
    pub data: CommandData,
}

impl CommandContext {
    pub fn new(bot: Arc<Bot>, env: Env, data: CommandData) -> Self {
        Self {
            bot: bot, 
            env: env,
            data: data
        }
    }

    pub (crate) fn with_data(self, inner_data: CommandData) -> Self {
        Self::new(self.bot, self.env, inner_data)
    }
}