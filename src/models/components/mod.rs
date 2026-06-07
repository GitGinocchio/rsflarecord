use async_trait::async_trait;

use crate::models::interaction::Interaction;
use crate::error::Result;

pub mod data;

pub type ComponentType = Box<dyn Component>;

#[async_trait(?Send)]
pub trait Component: Send + Sync {
    fn id(&self) -> String;

    fn build(&self) -> ();

    async fn handle(&self, interaction: Interaction) -> Result<()>;
}