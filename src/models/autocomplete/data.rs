use twilight_model::{
    application::{
        command::CommandType as TwilightCommandType, 
        interaction::application_command::{
            CommandData as TwilightCommandData, 
            CommandOptionValue as TwilightCommandOptionValue
        }
    }, 
    id::{Id, 
        marker::{
            CommandMarker, 
            GenericMarker, 
            GuildMarker
        }
    }
};

#[allow(unused)]
pub struct AutocompleteData(pub (crate) TwilightCommandData);

impl AutocompleteData {
    /// ID of the guild the command is registered to.
    pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.0.guild_id
    }

    /// If this is a user or message command, the ID of the targeted user/message.
    pub fn target_id(&self) -> Option<Id<GenericMarker>> {
        self.0.target_id
    }

    pub fn command_id(&self) -> Id<CommandMarker> {
        self.0.id
    }

    pub fn command_name(&self) -> &String {
        &self.0.name
    }

    pub fn command_type(&self) -> TwilightCommandType {
        self.0.kind
    }

    pub (crate) fn get_subcommand_name(&self) -> Option<&str> {
        self.0.options.iter().find_map(|opt| match opt.value {
            TwilightCommandOptionValue::SubCommand(_) => {
                Some(opt.name.as_str())
            }
            _ => None
        })
    }

    pub (crate) fn get_subcommand_group_name(&self) -> Option<&str> {
        self.0.options.iter().find_map(|opt| match opt.value {
            TwilightCommandOptionValue::SubCommandGroup(_) => {
                Some(opt.name.as_str())
            }
            _ => None,
        })
    }
}

impl From<TwilightCommandData> for AutocompleteData {
    fn from(value: TwilightCommandData) -> Self {
        Self(value)
    }
}