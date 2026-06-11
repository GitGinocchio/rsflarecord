use async_trait::async_trait;
use crate::error::BotResult;
use crate::models::modals::context::ModalContext;
use crate::models::modals::interaction::ModalInteraction;

pub mod interaction;
pub mod context;
pub mod data;

pub type ModalType = Box<dyn Modal>;

#[async_trait(?Send)]
#[allow(unused)]
pub trait Modal: Send + Sync {
    fn id(&self) -> String;

    fn title(&self) -> String;

    fn components(&self) -> Vec<()>;

    async fn on_submit(&self, interaction: ModalInteraction, ctx: ModalContext) -> BotResult<()>;
}