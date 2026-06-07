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
        let user = ctx.data.get_resolved_user("user");

        let message = match user {
            Some(user) => format!("Hello {0}, {1} greeted you", user.mention(), author.mention()),
            None => format!("Hello {0}!", author.name)
        };
        
        Ok(CommandResponseBuilder::new()
            .content(message)
            .ephemeral()
            .build())
    }
}