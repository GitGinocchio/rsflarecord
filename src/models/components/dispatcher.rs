use crate::models::components::{ComponentType, context::ComponentContext, interaction::ComponentInteraction};


pub (crate) struct ComponentDispatcher;

impl ComponentDispatcher {
    pub (crate) fn dispatch(component: &ComponentType, interaction: ComponentInteraction, ctx: ComponentContext) {
    }
}