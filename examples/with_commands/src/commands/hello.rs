use flarecord::prelude::*;
use flarecord::error::Error;
use async_trait::async_trait;

pub struct Hello;

#[async_trait(?Send)]
impl Command for Hello {
    fn name(&self) -> String {
        "hello".into()
    }

    fn description(&self) -> String {
        "Say Hi to someone in chat!".into()
    }

    fn options(&self) -> CommandOptions {
        let user_option = CommandOptionBuilder::user("user", "the user to greet")
            .build()?;
       
        Ok(Some(vec![user_option]))
    }

    async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult {
        let author = interaction.author().ok_or(Error::Generic("Missing author".into()))?;
        let user = match ctx.data.get_user_option("user")? {
            Some(user_id) => Some(ctx.discord.fetch_user(&user_id).await?),
            None => None
        };

        let message = match user {
            Some(user) => format!("Hello {0}, {1} greeted you", user.name, author.name),
            None => format!("Hello {0}!", author.name)
        };
        
        Ok(CommandResponseBuilder::new()
            .content(message)
            .ephemeral()
            .build())
    }
}