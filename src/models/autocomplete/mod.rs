use crate::models::autocomplete::response::AutocompleteResponse;

pub (crate) mod dispatcher;

pub mod response;
pub mod interaction;
pub mod context;
pub mod value;
pub mod data;

pub type AutocompleteResult = crate::error::Result<AutocompleteResponse>;