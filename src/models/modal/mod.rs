use async_trait::async_trait;
use worker::Env;

use crate::models::interaction::Interaction;
use crate::error::Result;

pub type ModalType = Box<dyn Modal>;

#[async_trait]
#[allow(unused)]
pub trait Modal: Send + Sync {
    fn id(&self) -> String;

    fn title(&self) -> String;

    fn components(&self) -> Vec<()>;

    async fn on_submit(&self, interaction: Interaction, data: (), env: Env) -> Result<()>;
}