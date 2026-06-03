use rsflarecord::models::command::{Command, MaybeCommandResult};
use async_trait::async_trait;


pub struct Hello;

#[async_trait]
impl Command for Hello {
    fn name(&self) -> String {
        "hello".into()
    }

    fn description(&self) -> String {
        "Say Hi to someone in chat!".into()
    }

    async fn execute(&self, interaction: (), env: worker::Env) -> MaybeCommandResult {
        None
    }
}