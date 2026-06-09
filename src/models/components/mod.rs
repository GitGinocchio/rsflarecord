use async_trait::async_trait;

use crate::models::command::response::CommandResponse;
use crate::models::components::context::ComponentContext;
use crate::models::components::interaction::ComponentInteraction;
use crate::error::BotResult;

pub (crate) mod dispatcher;
pub mod context;
pub mod interaction;
pub mod data;

pub type ComponentType = Box<dyn Component>;

#[async_trait(?Send)]
pub trait Component: Send + Sync {
    fn id(&self) -> String;

    fn build(&self) -> ();

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse>;
}