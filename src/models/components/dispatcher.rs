use crate::{error::BotResult, models::{command::{Command, response::CommandResponse}, components::{Component, CustomComponentType, context::ComponentContext, interaction::ComponentInteraction}}};


pub (crate) struct ComponentDispatcher;

impl ComponentDispatcher {
    pub (crate) async fn dispatch(
        component: &CustomComponentType, 
        interaction: ComponentInteraction, 
        ctx: ComponentContext
    ) -> BotResult<CommandResponse> {
        if component.id() == interaction.data.custom_id {
            return component.handle(interaction, ctx).await;
        }

        let root = component.build();

        

        Ok(CommandResponse::new())
    }
}