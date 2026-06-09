use async_trait::async_trait;
use crate::error::Result;
use crate::models::modal::context::ModalContext;
use crate::models::modal::interaction::ModalInteraction;

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

    async fn on_submit(&self, interaction: ModalInteraction, ctx: ModalContext) -> Result<()>;
}