use twilight_model::{application::interaction::application_command::CommandOptionValue as TwilightCommandOptionValue, id::{Id, marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker}}};

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

impl From<&TwilightCommandOptionValue> for CommandOptionValue {
    fn from(value: &TwilightCommandOptionValue) -> Self {
        match value {
            TwilightCommandOptionValue::Attachment(id) => Self::Attachment(*id),
            TwilightCommandOptionValue::Boolean(b) => Self::Boolean(*b),
            TwilightCommandOptionValue::Channel(id) => Self::Channel(*id),
            TwilightCommandOptionValue::Integer(i) => Self::Integer(*i),
            TwilightCommandOptionValue::String(s) => Self::String(s.clone()),
            TwilightCommandOptionValue::User(id) => Self::User(*id),
            TwilightCommandOptionValue::Role(id) => Self::Role(*id),
            TwilightCommandOptionValue::Number(n) => Self::Number(*n),
            TwilightCommandOptionValue::Mentionable(id) => Self::Mentionable(*id),
            
            TwilightCommandOptionValue::SubCommand(_) | 
            TwilightCommandOptionValue::SubCommandGroup(_) |
            TwilightCommandOptionValue::Focused(_, _)  => {
                unreachable!("Discord API contract violation: unexpected option type in command")
            }
        }
    }
}

macro_rules! impl_option_accessors {
    ($($as_name:ident, $get_name_option:ident, $target:ty, $variant:path, $variant_name:expr);* $(;)?) => {
        use crate::prelude::CommandData;
        impl CommandData {
            $(
                /// Attempts to retrieve an optional command option.
                ///
                /// Returns `None` if the option is not present, 
                /// `Some(value)` if present
                pub fn $get_name_option(&self, name: &str) -> Option<$target> {
                    let option = match self.0.options.iter().find(|opt| opt.name == name) {
                        Some(opt) => opt,
                        None => return None,
                    };

                    let val = CommandOptionValue::from(&option.value);

                    val.$as_name()
                }
            )*
        }

        impl CommandOptionValue {
            $(
                pub fn $as_name(self) -> Option<$target> {
                    if let $variant(val) = self {
                        Some(val)
                    } else {
                        None
                    }
                }
            )*
        }
    };
}

impl_option_accessors!(
    as_attachment,  get_option_attachment,  Id<AttachmentMarker>, CommandOptionValue::Attachment, "attachment";
    as_role,        get_option_role,        Id<RoleMarker>,       CommandOptionValue::Role,       "role";
    as_user,        get_option_user,        Id<UserMarker>,       CommandOptionValue::User,       "user";
    as_channel,     get_option_channel,     Id<ChannelMarker>,    CommandOptionValue::Channel,    "channel";
    as_mentionable, get_option_mentionable, Id<GenericMarker>,    CommandOptionValue::Mentionable,"mentionable";
    as_number,      get_option_number,      f64,                  CommandOptionValue::Number,     "number";
    as_integer,     get_option_integer,     i64,                  CommandOptionValue::Integer,    "integer";
    as_string,      get_option_string,      String,               CommandOptionValue::String,     "string";
    as_boolean,     get_option_boolean,     bool,                 CommandOptionValue::Boolean,    "boolean";
);