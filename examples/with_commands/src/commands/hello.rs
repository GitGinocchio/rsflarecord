use flarecord::{models::command::context::CommandContext, prelude::*};
use flarecord::error::Error;
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

    fn options(&self) -> CommandOptions {
        Some(vec![
            CommandOption::user("user", "the user to greet")
        ])
    }

    async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult {
        let user_id = ctx.data.get_user_option("user")?.ok_or(Error::MissingOption("user".into()))?;
        let user = ctx.discord.fetch_user(&user_id).await?;
        
        let author = interaction.author().ok_or(Error::Generic("Missing author".into()))?;
        
        Ok(CommandResponseBuilder::new()
            .content(format!("Hello {0}, {1} greeted you", user.name, author.name))
            .ephemeral()
            .build())
    }
}