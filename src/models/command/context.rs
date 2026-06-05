use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use worker::Env;

use crate::{
    bot::Bot, 
    models::command::{
        CommandResult, 
        data::CommandData
    }
};



pub struct CommandContext {
    pub bot: Arc<RwLock<Bot>>,
    pub env: Env,
    pub data: CommandData,
}

impl CommandContext {
    pub fn new(bot: Arc<RwLock<Bot>>, env: Env, data: CommandData) -> Self {
        Self {
            bot: bot, 
            env: env,
            data: data
        }
    }

    pub fn with_data(self, inner_data: CommandData) -> Self {
        Self::new(self.bot, self.env, inner_data)
    }
}

#[async_trait]
pub trait Command {
    async fn execute(&self, ctx: CommandContext) -> CommandResult;
}