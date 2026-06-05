use std::{ops::Deref};
use twilight_model::{application::interaction::{
    Interaction as TwilightInteraction, InteractionData, InteractionType
}, http::interaction::{InteractionResponse, InteractionResponseType}};
use worker::{Env, Response};

use crate::{bot::Bot, error::{Error, Result}, models::{autocomplete::dispatcher::AutocompleteDispatcher, command::{context::CommandContext, data::CommandData, dispatcher::CommandDispatcher}, components::data::ComponentData, modal::data::ModalData}};


pub struct Interaction(TwilightInteraction);

#[allow(unused)]
impl Interaction {
    pub async fn perform(self, env: Env) -> Result<Response> {
        match self.kind {
            InteractionType::ApplicationCommandAutocomplete => self.handle_autocomplete(env).await,
            InteractionType::ApplicationCommand => self.handle_command(env).await,
            InteractionType::MessageComponent => self.handle_component().await,
            InteractionType::ModalSubmit => self.handle_modal_submit(env).await,
            InteractionType::Ping => self.handle_ping().await,
            _ => Ok(Response::empty()?)
        }
    }

    async fn handle_ping(&self) -> Result<Response> {
        let response = InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None
        };
        
        let value = serde_json::to_value(response)?;
        Response::from_json(&value).map_err(Error::WorkerError)
    }

    async fn handle_command(self, env: Env) -> Result<Response> {
        let mut data = match self.data.as_ref() {
            Some(InteractionData::ApplicationCommand(data)) => CommandData::from(*data.clone()),
            Some(_) | None => return Err(Error::InvalidPayload("Missing or invalid command data".into()))
        };
        
        let bot = Bot::get_global();
        let bot_guard = bot.read().map_err(|e| Error::LockPoisoned)?;

        let Some(command) = bot_guard.commands.get(&data.0.name) else {
            return Err(Error::CommandNotFound(format!("{}", data.0.name)))
        };

        let ctx = CommandContext::new(bot.clone(), env, data);

        match CommandDispatcher::dispatch(command, self, ctx).await {
            Err(e) => Ok(e.as_response()?),
            Ok(response) => {
                let value = serde_json::to_value::<InteractionResponse>(response.into())
                    .map_err(Error::JsonFailed)?;

                Response::from_json(&value).map_err(Error::WorkerError)
            }
        }
    }

    async fn handle_autocomplete(self, env: Env) -> Result<Response> {
        let mut data = match self.data.as_ref() {
            Some(InteractionData::ApplicationCommand(data)) => CommandData::from(*data.clone()),
            Some(_) | None => return Err(Error::InvalidPayload("Missing or invalid command data".into()))
        };

        let bot = Bot::get_global();
        let bot_guard = bot.read().map_err(|e| Error::LockPoisoned)?;

        let Some(command) = bot_guard.commands.get(&data.0.name) else {
            return Err(Error::CommandNotFound(format!("{}", data.0.name)))
        };

        match AutocompleteDispatcher::dispatch(command, self, data, env).await {
            Err(e) => Ok(e.as_response()?),
            Ok(response) => {
                let value = serde_json::to_value::<InteractionResponse>(response.into())
                    .map_err(Error::JsonFailed)?;

                Response::from_json(&value).map_err(Error::WorkerError)
            },
        }
    }

    async fn handle_modal_submit(self, env: Env) -> Result<Response> {
        let mut data = match self.data.as_ref() {
            Some(InteractionData::ModalSubmit(data)) => ModalData::from(*data.clone()),
            Some(_) | None => return Err(Error::InvalidPayload("Missing or invalid modal data".into()))
        };

        let bot = Bot::get_global();
        let bot_guard = bot.read().map_err(|e| Error::LockPoisoned)?;

        let Some(modal) = bot_guard.modals.get(&data.custom_id) else {
            return Err(Error::ModalNotFound(format!("{}", data.custom_id)))
        };

        match modal.on_submit(self, data, env).await {
            Ok(response) => Ok(Response::empty()?),
            Err(e) => Ok(e.as_response()?)
        }
    }

    async fn handle_component(&self) -> Result<Response> {
        let mut data = match self.data.as_ref() {
            Some(InteractionData::MessageComponent(data)) => ComponentData::from(*data.clone()),
            Some(_) | None => return Err(Error::InvalidPayload("Missing or invalid modal data".into()))
        };

        let bot = Bot::get_global();
        let bot_guard = bot.read().map_err(|e| Error::LockPoisoned)?;

        let Some(component) = bot_guard.components.get(&data.custom_id) else {
            return Err(Error::ComponentNotFound(format!("{}", data.custom_id)))
        };

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