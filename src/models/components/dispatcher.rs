use crate::{error::BotResult, models::{command::response::CommandResponse, components::{ComponentType, context::ComponentContext, interaction::ComponentInteraction}}};


pub (crate) struct ComponentDispatcher;

impl ComponentDispatcher {
    pub (crate) async fn dispatch(
        component: &ComponentType, 
        interaction: ComponentInteraction, 
        ctx: ComponentContext
    ) -> BotResult<CommandResponse> {
        if component.id() == interaction.data.custom_id {
            return component.handle(interaction, ctx).await;
        }

        Ok(CommandResponse::new())
    }
}