use std::collections::HashMap;

use super::{AutocompleteResponse, AutocompleteValue};


pub struct AutocompleteResponseBuilder(AutocompleteResponse);

impl AutocompleteResponseBuilder {
    pub fn new() -> Self {
        Self(AutocompleteResponse::new())
    }

    pub fn build(self) -> AutocompleteResponse {
        self.0
    }

    pub fn choice(mut self, name: impl Into<String>, value: AutocompleteValue, locals: Option<HashMap<String, String>>) -> Self {
        self.0.add(name, value, locals);
        self
    }
}