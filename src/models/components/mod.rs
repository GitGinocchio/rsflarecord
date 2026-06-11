use async_trait::async_trait;

use crate::models::command::response::CommandResponse;
use crate::models::components::context::ComponentContext;
use crate::models::components::interaction::ComponentInteraction;
use crate::error::BotResult;
use crate::models::components::layout::RootComponent;

pub (crate) mod dispatcher;
pub mod context;
pub mod interaction;
pub mod data;

pub mod layout;
pub mod interactive;

pub type ComponentType = Box<dyn Component>;

/*
Concetti da tenere a mente per gestire i componenti:
Chi usa il framework crea componenti con degli ID testuali di lunghezza indefinita
poi questi componenti vengono aggiunti nel BotBuilder in un array di componenti
e per identificarli vengono utilizzati gli indici dei componenti
L'utente invece per capire da quale componente proviene puo' utilizzare il proprio ID
in quanto poi ottenendo il componente specifico sara' possibile fare .id() / .name()

Devo cercare un pattern che unisca:
#[async_trait(?Send)]
pub trait Component: Send + Sync {
    fn id(&self) -> String;

    fn build(&self) -> ...;

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse>;
}

a questo:
let ui = Container::new()
    .add(ActionRow::new()
        .add(Button::new("pet_coyote", "Pet it!")
            .on_click(|ctx| async move { 
                ctx.reply("Hai accarezzato il coyote!").await 
            })
        )
    );


Soluzione migliore:
fn build(&self) -> Vec<LayoutComponent> {
    
}

ATTENZIONE: Qui ci vorrebbe non un Vec<LayoutComponent> ma un RootComponent, il quale puo' tenere:
- 1 Container
- 5 ActionRow

*/

#[async_trait(?Send)]
pub trait Component: Send + Sync {
    fn id(&self) -> String;

    fn build(&self) -> RootComponent;

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse>;
}