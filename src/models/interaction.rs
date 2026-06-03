use std::ops::Deref;
use twilight_model::application::interaction::Interaction as TwilightInteraction;
use worker::Response;

use crate::{bot::Bot, error::Result};


pub struct Interaction(TwilightInteraction);

#[allow(unused)]
impl Interaction {
    pub fn perform(&self, bot: &Bot) -> Result<Response> {

        Ok(Response::empty()?)
    }

    async fn handle_command(&self) {

    }

    async fn handle_autocomplete(&self) {

    }

    async fn handle_modal_submit(&self) {

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