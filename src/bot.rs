use std::collections::HashMap;

use twilight_model::application::interaction::Interaction as TwilightInteraction;
use worker::{Env, Request, Response};

use crate::models::command::data::CommandData;
use crate::models::command::CommandHandler;
use crate::models::command::{CommandType, MaybeCommandResult};
use crate::error::{Error, Result};
use crate::crypto;
use crate::models::interaction::Interaction;

#[allow(unused)]
pub struct Bot {
    pub commands: HashMap<String, CommandType>
}

#[allow(unused)]
impl Bot {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new()
        }
    }

    pub fn register_command(&mut self, command: CommandType) -> Result<()> {
        self.commands.insert(command.name().into(), command);
        Ok(())
    }

    pub fn register_commands(&mut self, commands: Vec<CommandType>) -> Result<()> {
        for command in commands {
            self.register_command(command)?;
        }
        Ok(())
    }

    pub fn register_command_handler<F, Fut>(&mut self, 
        name: impl Into<String>, 
        description: impl Into<String>, 
        handler: F
    ) -> Result<()> 
    where 
        F: Fn(Interaction, CommandData, Env) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = MaybeCommandResult> + Send + Sync + 'static,
    {
        let handler = CommandHandler::new(name.into(), description.into(), handler);

        self.register_command(Box::new(handler))?;

        Ok(())
    }

    pub async fn handle(&self, mut req: Request, env: Env) -> Result<Response> {
        let body = req.bytes().await?;
        let headers = req.headers();

        let public_key = env.secret("DISCORD_PUBLIC_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();
    
        let is_valid = crypto::verify_signature(headers, &body, &public_key)?;

        if !is_valid {
            return Response::error("Unauthorized", 401).map_err(Error::WorkerError);
        }

        let tw_interaction: TwilightInteraction = serde_json::from_slice(&body)?;
        let interaction = Interaction::from(tw_interaction);

        interaction.perform(self, env).await
    }
}