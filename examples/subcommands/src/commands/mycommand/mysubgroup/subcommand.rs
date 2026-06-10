use flarecord::{models::command::Subcommand, prelude::*};
use async_trait::async_trait;

pub struct MySubcommand;

#[async_trait(?Send)]
impl Subcommand for MySubcommand {
    fn name(&self) -> String {
        "mycommand".into()
    }

    fn description(&self) -> String {
        "My command that contains a subcommand".into()
    }

    // execute is required on subcommands!
    async fn execute(&self, _interaction: CommandInteraction, _ctx: CommandContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::new())
    }
}