use worker::Env;

use crate::{
    error::Error, 
    models::{
        command::{
            Command, 
            MaybeAutocompleteResult, 
            SubcommandGroup, 
            data::CommandData
        }, 
        interaction::Interaction
    }
};

pub (crate) struct AutocompleteDispatcher;

impl AutocompleteDispatcher {
    pub (crate) async fn dispatch(
        cmd: &Box<dyn Command>,
        interaction: Interaction,
        data: CommandData,
        env: Env,
    ) -> MaybeAutocompleteResult {
        if let Some(group_name) = data.get_subcommand_group_name() {
            if let Some(group) = cmd.groups().iter().find(|g| g.name() == group_name) {
                let Some(inner_data) = data.get_inner() else {
                    return Some(Err(Error::InvalidInteraction(format!("Missing inner data for the subgroup!"))));
                };

                return Self::dispatch_group(group, interaction, inner_data, env).await
            }
        }

        if let Some(sub_name) = data.get_subcommand_name() {
            if let Some(sub) = cmd.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_data) = data.get_inner() else {
                    return Some(Err(Error::InvalidInteraction(format!("Missing inner data for the subcommand!"))));
                };

                return Some(sub.autocomplete(interaction, inner_data, env).await?)
            }
        }

        cmd.autocomplete(interaction, data, env).await
    }

    async fn dispatch_group(
        group: &Box<dyn SubcommandGroup>,
        interaction: Interaction,
        data: CommandData,
        env: Env
    ) -> MaybeAutocompleteResult {
        if let Some(sub_name) = data.get_subcommand_name() {
            if let Some(sub) = group.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_data) = data.get_inner() else {
                    return Some(Err(Error::InvalidInteraction(format!("Missing inner data for the subcommand!"))));
                };

                return sub.autocomplete(interaction, inner_data, env).await;
            }
        }

        Some(Err(Error::CommandNotFound("Subcommand not found in group".into())))
    }
}