use std::ops::Deref;
use twilight_model::{application::interaction::{
    Interaction as TwilightInteraction, InteractionData, InteractionType
}, http::interaction::{InteractionResponse, InteractionResponseType}};
use worker::{Env, Response};

use crate::{bot::Bot, error::{Error, Result}, models::command::{data::CommandData, dispatcher::CommandDispatcher}};


pub struct Interaction(TwilightInteraction);

#[allow(unused)]
impl Interaction {
    pub async fn perform(self, bot: &Bot, env: Env) -> Result<Response> {
        match self.kind {
            InteractionType::ApplicationCommandAutocomplete => self.handle_autocomplete(bot).await,
            InteractionType::ApplicationCommand => self.handle_command(bot, env).await,
            InteractionType::MessageComponent => self.handle_component(bot).await,
            InteractionType::ModalSubmit => self.handle_modal_submit(bot).await,
            InteractionType::Ping => self.handle_ping(bot).await,
            _ => Ok(Response::empty()?)
        }
    }

    async fn handle_ping(&self, bot: &Bot) -> Result<Response> {
        let response = InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None
        };
        
        let value = serde_json::to_value(response)?;
        Response::from_json(&value).map_err(Error::WorkerError)
    }

    async fn handle_command(self, bot: &Bot, env: Env) -> Result<Response> {
        let mut data = match self.data.as_ref() {
            Some(InteractionData::ApplicationCommand(data)) => CommandData::from(*data.clone()),
            Some(_) | None => return Err(Error::InvalidPayload("Missing or invalid command data".into()))
        };

        let Some(command) = bot.commands.get(&data.name) else {
            return Err(Error::CommandNotFound(format!("{}", data.name)))
        };

        match CommandDispatcher::dispatch(command, self, data, env).await {
            Some(Err(e)) => Ok(e.as_response()?),
            Some(Ok(response)) => {
                let value = serde_json::to_value::<InteractionResponse>(response.into())
                    .map_err(Error::JsonFailed)?;

                Response::from_json(&value).map_err(Error::WorkerError)
            },
            None => Response::empty().map_err(Error::WorkerError)
        }
    }

    async fn handle_component(&self, bot: &Bot) -> Result<Response> {
        Ok(Response::empty()?)
    }

    async fn handle_autocomplete(&self, bot: &Bot) -> Result<Response> {
        Ok(Response::empty()?)
    }

    async fn handle_modal_submit(&self, bot: &Bot) -> Result<Response> {
        Ok(Response::empty()?)
    }
}

impl From<TwilightInteraction> for Interaction {
    fn from(value: TwilightInteraction) -> Self {
        Self(value)
    }
}

impl Deref for Interaction {
    type Target = TwilightInteraction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}