use async_trait::async_trait;
use twilight_model::guild::Permissions;
use worker::Env;

use crate::{
    models::autocomplete::Autocomplete, 
    models::response::CommandResponse,
    error::Result
};

pub type CommandType = Box<dyn Command>;

pub type SubcommandType = Box<dyn Subcommand>;
pub type SubcommandGroupType = Box<dyn SubcommandGroup>;

pub type CommandResult = Result<CommandResponse>;
pub type MaybeCommandResult = Option<CommandResult>;

pub type AutocompleteResult = Result<Autocomplete>;
pub type MaybeAutocompleteResult = Option<AutocompleteResult>;

#[async_trait]
#[allow(unused)]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn default_member_permissions(&self) -> Option<Permissions> { None }

    async fn execute(&self, interaction: (), env: Env) -> MaybeCommandResult { None }
    async fn autocomplete(&self, interaction: (), env: Env) -> MaybeAutocompleteResult { None }

    fn subcommands(&self) -> Vec<SubcommandType> { vec![] }
    fn groups(&self) -> Vec<SubcommandGroupType> { vec![] }
}

#[async_trait]
#[allow(unused)]
pub trait Subcommand: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn default_member_permissions(&self) -> Option<Permissions> { None }

    async fn execute(&self, interaction: (), env: Env) -> CommandResult;
    async fn autocomplete(&self, interaction: (), env: Env) -> MaybeAutocompleteResult { None }
}

#[async_trait]
#[allow(unused)]
pub trait SubcommandGroup: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn subcommands(&self) -> Vec<SubcommandType> { vec![] }
}