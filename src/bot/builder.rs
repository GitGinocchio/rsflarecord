use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::{bot::Bot, models::{command::CommandType, components::ComponentType, modal::ModalType}};


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

    pub fn register_command(mut self, command: CommandType) -> Self {
        self.commands.insert(command.name().into(), command);
        self
    }

    pub fn build(self) -> Arc<RwLock<Bot>> {
        Bot::from(self).set_global();

        Bot::get_global()
    }
}