use twilight_model::{
    application::{
        command::CommandType as TwilightCommandType, 
        interaction::{
            InteractionChannel, InteractionDataResolved, InteractionMember, application_command::{
                CommandData as TwilightCommandData, 
                CommandOptionValue as TwilightCommandOptionValue
            }
        }
    }, channel::{Attachment, Message}, guild::Role, id::{
        Id, 
        marker::{
            CommandMarker, GenericMarker, GuildMarker
        }
    }
};

use crate::{models::{command::option::value::CommandOptionValue, user::UserRef}};

pub struct CommandData(pub (crate) TwilightCommandData);

#[allow(unused)]
impl CommandData {
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

    pub fn resolved(&self) -> &Option<InteractionDataResolved> {
        &self.0.resolved
    }

    /// Retrieves a resolved user by the option name.
    pub fn get_resolved_user<'a>(&'a self, name: &str) -> Option<UserRef<'a>> {
        let id = self.get_option_user(name)?;
        self.0.resolved.as_ref()?.users.get(&id).map(UserRef::from)
    }

    /// Retrieves a resolved member by the option name.
    pub fn get_resolved_member<'a>(&'a self, name: &str) -> Option<&'a InteractionMember> {
        // Members are indexed by user ID in the resolved payload
        let id = self.get_option_user(name)?;
        self.0.resolved.as_ref()?.members.get(&id)
    }

    /// Retrieves a resolved role by the option name.
    pub fn get_resolved_role<'a>(&'a self, name: &str) -> Option<&'a Role> {
        let id = self.get_option_role(name)?;
        self.0.resolved.as_ref()?.roles.get(&id)
    }

    /// Retrieves a resolved message by the option name.
    pub fn get_resolved_message<'a>(&'a self, name: &str) -> Option<&'a Message> {
        let id = self.get_option_mentionable(name)?; // Assicurati di avere questo metodo nella macro
        self.0.resolved.as_ref()?.messages.get(&id.cast())
    }

    /// Retrieves a resolved channel by the option name.
    pub fn get_resolved_channel<'a>(&'a self, name: &str) -> Option<&'a InteractionChannel> {
        let id = self.get_option_channel(name)?;
        self.0.resolved.as_ref()?.channels.get(&id)
    }

    /// Retrieves a resolved attachment by the option name.
    pub fn get_resolved_attachment<'a>(&'a self, name: &str) -> Option<&'a Attachment> {
        let id = self.get_option_attachment(name)?;
        self.0.resolved.as_ref()?.attachments.get(&id)
    }

    pub fn get_option(&self, name: &str) -> Option<CommandOptionValue> {
        let Some(option) = self.0.options.iter().find(|opt| opt.name == name) else {
            return None;
        };

        Some(CommandOptionValue::from(&option.value))
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

    pub (crate) fn get_inner(&self) -> Option<CommandData> {
        self.0.options.iter().find_map(|opt| {
            if let TwilightCommandOptionValue::SubCommand(sub_options) = &opt.value {
                Some(CommandData(TwilightCommandData {
                    name: opt.name.clone(),
                    options: sub_options.clone(),
                    resolved: self.0.resolved.clone(),
                    guild_id: self.0.guild_id,
                    id: self.0.id,
                    kind: self.0.kind,
                    target_id: self.0.target_id
                }))
            } else {
                None
            }
        })
    }


}

impl From<TwilightCommandData> for CommandData {
    fn from(value: TwilightCommandData) -> Self {
        Self(value)
    }
}