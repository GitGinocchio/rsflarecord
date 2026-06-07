use serde::{Serialize, Serializer};
use twilight_model::{
    application::command::{
        Command as TwilightCommand, 
        CommandOption as TwilightCommandOption, 
        CommandOptionType, 
        CommandType as TwilightCommandType
    }, 
    id::Id
};
use serde::ser::Error as SerdeError;

use crate::models::command::CommandType;


pub (crate) struct SerializableCommand<'a>(pub &'a CommandType);

impl<'a> Serialize for SerializableCommand<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut all_options: Vec<TwilightCommandOption> = self.0.options()
            .map_err(|e| SerdeError::custom(format!("Error parsing options: {e}")))?
            .unwrap_or_default()
            .iter()
            .map(|opt| TwilightCommandOption::from(opt))
            .collect();

        let subs = self.0.subcommands();
        if !subs.is_empty() {
            for sub in subs.iter() {
                let options: Vec<TwilightCommandOption> = sub.options()
                    .map_err(|e| SerdeError::custom(format!("Error parsing options: {e}")))?
                    .unwrap_or_default()
                    .iter()
                    .map(|opt| TwilightCommandOption::from(opt))
                    .collect();

                all_options.push(TwilightCommandOption {
                    kind: CommandOptionType::SubCommand,
                    name: sub.name(),
                    description: sub.description(),
                    options: if options.len() > 0 { Some(options) } else { None },
                    autocomplete: Some(true),
                    channel_types: None,
                    choices: None,
                    description_localizations: None,
                    max_length: None,
                    max_value: None,
                    min_length: None,
                    min_value: None,
                    name_localizations: None,
                    required: None
                });
            }
        }

        //let itypes = self.0.integration_types();
        //let icontexts = self.0.interaction_contexts();

        let discord_cmd = TwilightCommand {
            name: self.0.name(),
            description: self.0.description(),
            options: all_options,
            kind: TwilightCommandType::ChatInput,
            application_id: None,
            default_member_permissions: self.0.default_member_permissions(),
            guild_id: None,
            id: None,
            nsfw: None,
            version: Id::new(1),
            name_localizations: None,
            description_localizations: None,
            contexts: None,
            /*
            contexts: if icontexts.is_empty() {
                None
            } else {
                Some(icontexts)
            },
            */
            #[allow(deprecated)]
            dm_permission: None,
            integration_types: None
            /*
            integration_types: if itypes.is_empty() {
                None
            } else {
                Some(itypes)
            },
            */
        };

        discord_cmd.serialize(serializer)
    }
}