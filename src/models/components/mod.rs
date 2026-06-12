use std::sync::Arc;

use async_trait::async_trait;

use crate::bot::Bot;
use crate::models::command::response::CommandResponse;
use crate::models::components::context::ComponentContext;
use crate::models::components::interaction::ComponentInteraction;
use crate::error::BotResult;
use crate::models::components::layout::{LayoutComponent, RootComponent};
use crate::traits::component::IntoComponent;

pub (crate) mod dispatcher;
pub mod context;
pub mod interaction;
pub mod content;
pub mod data;

pub mod layout;
pub mod interactive;

pub type CustomComponentType = Arc<dyn Component>;

pub enum ComponentType {
    Base(LayoutComponent),
    Custom(CustomComponentType)
}

impl<C: Component + 'static> IntoComponent for C {
    fn into_component(self) -> ComponentType {
        let bot = Bot::get_global();

        let component_id = self.id();

        let component = bot.components.get(&component_id)
            .expect("Component must be registered!");
        
        ComponentType::Custom(component.clone())
    }
}

impl IntoComponent for LayoutComponent {
    fn into_component(self) -> ComponentType {
        ComponentType::Base(self)
    }
}

#[async_trait(?Send)]
pub trait Component: Send + Sync {
    fn id(&self) -> String;

    fn build(&self) -> RootComponent;

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse>;
}