use std::ops::Deref;
use twilight_model::application::interaction::{
    Interaction as TwilightInteraction, 
    InteractionType
};
use worker::Response;

use crate::{bot::Bot, error::Result};


pub struct Interaction(TwilightInteraction);

#[allow(unused)]
impl Interaction {
    pub async fn perform(&self, bot: &Bot) -> Result<Response> {
        match self.kind {
            InteractionType::ApplicationCommand => self.handle_command(bot).await,
            InteractionType::MessageComponent => self.handle_component(bot).await,
            InteractionType::ModalSubmit => self.handle_modal_submit(bot).await,
            InteractionType::ApplicationCommandAutocomplete => self.handle_autocomplete(bot).await,
            InteractionType::Ping => self.handle_ping(bot).await,
            _ => Ok(Response::empty()?)
        }
    }

    async fn handle_ping(&self, bot: &Bot) -> Result<Response> {
        Ok(Response::empty()?)
    }

    async fn handle_command(&self, bot: &Bot) -> Result<Response> {
        Ok(Response::empty()?)
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