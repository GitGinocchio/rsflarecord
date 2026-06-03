use rsflarecord::models::command::{Command, MaybeCommandResult};
use async_trait::async_trait;


pub struct Hello;

#[async_trait]
impl Command for Hello {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn description(&self) -> &'static str {
        "Saluta qualcuno nella chat!"
    }

    async fn execute(&self, interaction: (), env: worker::Env) -> MaybeCommandResult {
        None
    }
}