use worker::{Env, Request, Response};

use crate::models::command::CommandType;
use crate::error::{Error, Result};
use crate::crypto;

#[allow(unused)]
pub struct Bot {
    commands: Vec<CommandType>
}

#[allow(unused)]
impl Bot {
    pub fn new() -> Self {
        Self {
            commands: vec![]
        }
    }

    pub fn register_command(&mut self, command: CommandType) -> Result<()> {
        Ok(())
    }

    pub fn register_commands(&mut self, commands: Vec<CommandType>) -> Result<()> {
        Ok(())
    }

    pub async fn handle(&self, mut req: Request, env: Env) -> Result<Response> {
        let body = req.bytes().await?;
        let headers = req.headers();

        let public_key = env.secret("DISCORD_PUBLIC_KEY")
            .map_err(Error::Environment)?
            .to_string();
    
        let is_valid = crypto::verify_signature(headers, &body, &public_key)?;

        if !is_valid {
            return Response::error("Unauthorized", 401).map_err(Error::Environment);
        }

        Response::empty().map_err(Error::Environment)
    }
}