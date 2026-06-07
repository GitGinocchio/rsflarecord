use twilight_model::{application::interaction::application_command::CommandOptionValue as TwilightCommandOptionValue, id::{Id, marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker}}};

use crate::error::{Error, Result};

pub enum CommandOptionValue {
    Attachment(Id<AttachmentMarker>),
    Boolean(bool),
    User(Id<UserMarker>),
    Role(Id<RoleMarker>),
    Channel(Id<ChannelMarker>),
    Mentionable(Id<GenericMarker>),
    Number(f64),
    Integer(i64),
    String(String)
}

impl TryFrom<TwilightCommandOptionValue> for CommandOptionValue {
    type Error = Error;

    fn try_from(value: TwilightCommandOptionValue) -> Result<Self> {
        match value {
            TwilightCommandOptionValue::Attachment(id) => Ok(Self::Attachment(id)),
            TwilightCommandOptionValue::Boolean(b) => Ok(Self::Boolean(b)),
            TwilightCommandOptionValue::Channel(id) => Ok(Self::Channel(id)),
            TwilightCommandOptionValue::Integer(i) => Ok(Self::Integer(i)),
            TwilightCommandOptionValue::String(s) => Ok(Self::String(s)),
            TwilightCommandOptionValue::User(id) => Ok(Self::User(id)),
            TwilightCommandOptionValue::Role(id) => Ok(Self::Role(id)),
            TwilightCommandOptionValue::Number(n) => Ok(Self::Number(n)),
            TwilightCommandOptionValue::Mentionable(id) => Ok(Self::Mentionable(id)),
            
            TwilightCommandOptionValue::SubCommand(_) | 
            TwilightCommandOptionValue::SubCommandGroup(_) |
            TwilightCommandOptionValue::Focused(_, _)  => {
                Err(Error::InvalidPayload(format!("Not a valid CommandOptionValue type!")))
            }
        }
    }
}

macro_rules! impl_option_accessors {
    ($($as_name:ident, $get_name_option:ident, $target:ty, $variant:path, $variant_name:expr);* $(;)?) => {
        use crate::prelude::CommandData;
        impl CommandData {
            $(
                pub fn $get_name_option(&self, name: &str) -> Result<Option<$target>> {
                    let option = match self.0.options.iter().find(|opt| opt.name == name) {
                        Some(opt) => opt,
                        None => return Ok(None),
                    };

                    let val = CommandOptionValue::try_from(option.value.clone())?;

                    Ok(Some(val.$as_name()?))
                }
            )*
        }

        impl CommandOptionValue {
            $(
                pub fn $as_name(self) -> Result<$target> {
                    self.try_into()
                }
            )*
        }

        $(
            impl TryInto<$target> for CommandOptionValue {
                type Error = Error;

                fn try_into(self) -> Result<$target> {
                    if let $variant(val) = self {
                        Ok(val)
                    } else {
                        Err(Error::InvalidPayload(format!("Not a {}", $variant_name)))
                    }
                }
            }
        )*
    };
}

// TODO: aggiungere per ognuno anche:
// get_user -> Result<Id<UserMarker>>
// get_attachment -> ...
// ...

impl_option_accessors!(
    as_attachment,  get_attachment_option,  Id<AttachmentMarker>, CommandOptionValue::Attachment, "attachment";
    as_role,        get_role_option,        Id<RoleMarker>,       CommandOptionValue::Role,       "role";
    as_user,        get_user_option,        Id<UserMarker>,       CommandOptionValue::User,       "user";
    as_channel,     get_channel_option,     Id<ChannelMarker>,    CommandOptionValue::Channel,    "channel";
    as_mentionable, get_mentionable_option, Id<GenericMarker>,    CommandOptionValue::Mentionable,"mentionable";
    as_number,      get_number_option,      f64,                  CommandOptionValue::Number,     "number";
    as_integer,     get_integer_option,     i64,                  CommandOptionValue::Integer,    "integer";
    as_string,      get_string_option,      String,               CommandOptionValue::String,     "string";
    as_boolean,     get_boolean_option,     bool,                 CommandOptionValue::Boolean,    "boolean";
);