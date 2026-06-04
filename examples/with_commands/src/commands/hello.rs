use flarecord::models::{command::{Command, MaybeCommandResult, data::CommandData}, interaction::Interaction};
use async_trait::async_trait;
use worker::Env;


pub struct Hello;

#[async_trait]
impl Command for Hello {
    fn name(&self) -> String {
        "hello".into()
    }

    fn description(&self) -> String {
        "Say Hi to someone in chat!".into()
    }

    fn options(&self) -> Option<()> {
        None
    }

    async fn execute(&self, interaction: Interaction, data: CommandData, env: Env) -> MaybeCommandResult {
        None
    }
}