use std::collections::HashMap;

use twilight_model::{
    application::command::{
        CommandOptionChoice,
    }, 
    http::interaction::{
        InteractionResponse, 
        InteractionResponseData,
        InteractionResponseType
    }
};

use crate::models::autocomplete::value::AutocompleteValue;

pub mod builder;

pub struct AutocompleteResponse {
    choices: Vec<CommandOptionChoice>
}

impl AutocompleteResponse {
    pub fn new() -> Self {
        Self { choices: Vec::new() }
    }

    pub fn add(&mut self, name: impl Into<String>, value: impl Into<AutocompleteValue>, locals: Option<HashMap<String, String>>) { 
        self.choices.push(CommandOptionChoice {
            name: name.into(),
            name_localizations: locals,
            value: value.into().into()
        });
    }
}

impl Into<InteractionResponse> for AutocompleteResponse {
    fn into(self) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
            data: Some(InteractionResponseData {
                choices: Some(self.choices),
                ..Default::default()
            }),
        }
    }
}