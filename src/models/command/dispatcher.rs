use crate::{
    error::Error, 
    models::{
        command::{
            Command, 
            CommandResult, 
            SubcommandGroup, 
            context::CommandContext
        }, 
        interaction::Interaction
    }
};

pub (crate) struct CommandDispatcher;

impl CommandDispatcher {
    pub (crate) async fn dispatch(
        cmd: &Box<dyn Command>,
        interaction: Interaction,
        ctx: CommandContext
    ) -> CommandResult {
        if let Some(group_name) = ctx.data.get_subcommand_group_name() {
            if let Some(group) = cmd.groups().iter().find(|g| g.name() == group_name) {
                let Some(inner_data) = ctx.data.get_inner() else {
                    return Err(Error::InvalidInteraction(format!("Missing inner data for the subgroup!")));
                };

                let inner_ctx = ctx.with_data(inner_data);

                return Self::dispatch_group(group, interaction, inner_ctx).await
            }
        }

        if let Some(sub_name) = ctx.data.get_subcommand_name() {
            if let Some(sub) = cmd.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_data) = ctx.data.get_inner() else {
                    return Err(Error::InvalidInteraction(format!("Missing inner data for the subcommand!")));
                };

                let inner_ctx = ctx.with_data(inner_data);

                return sub.execute(interaction, inner_ctx).await
            }
        }

        cmd.execute(interaction, ctx).await
    }

    async fn dispatch_group(
        group: &Box<dyn SubcommandGroup>,
        interaction: Interaction,
        ctx: CommandContext
    ) -> CommandResult {
        if let Some(sub_name) = ctx.data.get_subcommand_name() {
            if let Some(sub) = group.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_data) = ctx.data.get_inner() else {
                    return Err(Error::InvalidInteraction(format!("Missing inner data for the subcommand!")));
                };

                let inner_ctx = ctx.with_data(inner_data);
                return sub.execute(interaction, inner_ctx).await
            }
        }

        Err(Error::CommandNotFound("Subcommand not found in group".into()))
    }
}