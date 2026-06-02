use std::collections::HashMap;

use twilight_model::{
    application::command::{
        CommandOptionChoice, 
        CommandOptionChoiceValue
    }, 
    http::interaction::{
        InteractionResponse, 
        InteractionResponseData,
        InteractionResponseType
    }
};

pub type AutocompleteValue = CommandOptionChoiceValue;    

pub struct Autocomplete {
    pub choices: Vec<CommandOptionChoice>
}

impl Autocomplete {
    pub fn new() -> Self {
        Self { choices: Vec::new() }
    }

    pub fn add(&mut self, name: impl Into<String>, value: AutocompleteValue, locals: Option<HashMap<String, String>>) { 
        self.choices.push(CommandOptionChoice {
            name: name.into(),
            name_localizations: locals,
            value: value
        });
    }
}

impl Into<InteractionResponse> for Autocomplete {
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