use flarecord::prelude::*;
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

    fn options(&self) -> CommandOptions {
        Some(vec![
            CommandOption::user("user", "the user to greet")
        ])
    }

    async fn execute(&self, interaction: Interaction, data: CommandData, _env: Env) -> CommandResult {
        let message = match interaction.author() {
            Some(author) => format!("Hello {0} from {0}", author.name),
            None => format!("Hello!")
        };
        
        Ok(CommandResponseBuilder::new()
            .content(message)
            .ephemeral()
            .build())
    }
}