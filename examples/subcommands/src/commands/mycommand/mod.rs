use flarecord::{models::command::{SubcommandGroupType, SubcommandType}, prelude::*};
use async_trait::async_trait;

use crate::commands::mycommand::subcommand::MySubcommand;
use crate::commands::mycommand::mysubgroup::MySubcommandGroup;

pub mod mysubgroup;
pub mod subcommand;

pub struct MyCommand;

#[async_trait(?Send)]
impl Command for MyCommand {
    fn name(&self) -> String {
        "mycommand".into()
    }

    fn description(&self) -> String {
        "My command that contains a subcommand".into()
    }

    fn groups(&self) -> Vec<SubcommandGroupType> { vec![
        Box::new(MySubcommandGroup)
    ]}

    fn subcommands(&self) -> Vec<SubcommandType> { vec![
        Box::new(MySubcommand),
    ]}

    /* Execute method will not receive interactions anymore when using subcommands!
    async fn execute(&self, interaction: CommandInteraction, _ctx: CommandContext) -> BotResult<CommandResponse> {
        // ...
    }
    */
}