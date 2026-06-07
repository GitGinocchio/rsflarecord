use async_trait::async_trait;
use twilight_model::guild::Permissions;

use crate::{
    error::{Error, Result}, 
    models::{
        autocomplete::{context::AutocompleteContext, response::AutocompleteResponse}, 
        command::{
            context::CommandContext, option::CommandOption, response::CommandResponse
        }, 
        interaction::Interaction,
    }
};

pub mod data;
pub mod response;
pub mod option;
pub mod dispatcher;
pub mod serializable;
pub mod context;


pub type CommandType = Box<dyn Command>;

pub type CommandOptions = Result<Option<Vec<CommandOption>>>;

pub type SubcommandType = Box<dyn Subcommand>;
pub type SubcommandGroupType = Box<dyn SubcommandGroup>;

pub type CommandResult = Result<CommandResponse>;
pub type AutocompleteResult = Result<AutocompleteResponse>;

#[async_trait(?Send)]
#[allow(unused)]
pub trait Command: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn subcommands(&self) -> Vec<SubcommandType> { vec![] }
    fn groups(&self) -> Vec<SubcommandGroupType> { vec![] }

    fn options(&self) -> CommandOptions { Ok(None) }

    async fn autocomplete(&self, interaction: Interaction, ctx: AutocompleteContext) -> AutocompleteResult {
        Err(Error::AutocompleteNotImplemented(self.name()))
    }

    async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult {
        Err(Error::ExecuteNotImplemented(self.name()))
    }
}

#[async_trait(?Send)]
#[allow(unused)]
pub trait Subcommand: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn options(&self) -> CommandOptions { Ok(None) }

    async fn autocomplete(&self, interaction: Interaction, ctx: AutocompleteContext) -> AutocompleteResult {
        Err(Error::AutocompleteNotImplemented(self.name()))
    }

    async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult;
}

#[async_trait(?Send)]
#[allow(unused)]
pub trait SubcommandGroup: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn subcommands(&self) -> Vec<SubcommandType> { vec![] }
}

pub struct CommandHandler<F, Fut> {
    pub name: String,
    pub description: String,
    pub handler: F,
    _marker: std::marker::PhantomData<Fut>,
}

impl<F, Fut> CommandHandler<F, Fut> {
    pub fn new(name: String, description: String, handler: F) -> Self {
        Self {
            name,
            description,
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<F, Fut> Command for CommandHandler<F, Fut> 
where 
    F: Fn(Interaction, CommandContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = CommandResult> + Send + Sync + 'static,
{
    fn name(&self) -> String { self.name.clone() }
    fn description(&self) -> String { self.description.clone() }

    async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult {
        (self.handler)(interaction, ctx).await
    }
}