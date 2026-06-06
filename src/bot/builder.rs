use std::{collections::HashMap, sync::Arc};

use crate::{bot::Bot, models::{command::{Command, CommandHandler, CommandResult, CommandType, context::CommandContext}, components::ComponentType, interaction::Interaction, modal::ModalType}};


#[allow(unused)]
pub struct BotBuilder {
    pub (crate) commands: HashMap<String, CommandType>,
    pub (crate) components: HashMap<String, ComponentType>,
    pub (crate) modals: HashMap<String, ModalType>
}

#[allow(unused)]
impl BotBuilder {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            components: HashMap::new(),
            modals: HashMap::new()
        }
    }

    pub fn register_command(mut self, command: impl Command + 'static) -> Self {
        self.commands.insert(command.name().into(), Box::new(command));
        self
    }

    pub fn register_command_handler<F, Fut>(mut self, 
        name: impl Into<String>, 
        description: impl Into<String>, 
        handler: F
    ) -> Self
    where 
        F: Fn(Interaction, CommandContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CommandResult> + Send + Sync + 'static,
    {
        let handler = CommandHandler::new(name.into(), description.into(), handler);
        self.commands.insert(handler.name.clone(), Box::new(handler));
        self
    }

    pub fn build(self) -> Arc<Bot> {
        Bot::from(self).set_global();

        Bot::get_global()
    }
}