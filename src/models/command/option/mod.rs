use twilight_model::{
    application::command::{
        CommandOption as TwilightCommandOption, 
        CommandOptionChoice, 
        CommandOptionChoiceValue, 
        CommandOptionType
    }, 
    channel::ChannelType
};

pub struct CommandOption {
    name: String,
    description: String,
    autocomplete: Option<bool>,
    channel_types: Option<Vec<ChannelType>>,
    choices: Option<Vec<CommandOptionChoice>>,
    required: Option<bool>,
    kind: CommandOptionType
}

impl CommandOption {
    fn new(name: impl Into<String>, description: impl Into<String>, kind: CommandOptionType) -> Self {
        Self {
            name: name.into(), 
            description: description.into(),
            kind: kind,
            autocomplete: None,
            channel_types: None,
            choices: None,
            required: None
        }
    }

    pub fn string(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::String)
    }

    pub fn bool(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::Boolean)
    }

    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::Integer)
    }

    pub fn number(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::Number)
    }

    pub fn mentionable(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::Mentionable)
    }

    pub fn user(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::User)
    }

    pub fn role(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::Role)
    }

    pub fn channel(name: impl Into<String>, description: impl Into<String>) -> Self {
        let mut command_option = Self::new(name, description, CommandOptionType::Channel);
        command_option.channel_types = Some(vec![]);

        command_option
    }

    pub fn attachment(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(name, description, CommandOptionType::Attachment)
    }

    pub fn set_required(&mut self, required: bool) {
        self.required = Some(required);
    }

    pub fn set_autocomplete(&mut self, autocomplete: bool) {
        self.autocomplete = Some(autocomplete);
    }

    pub fn set_channel_types(&mut self, channel_types: Vec<ChannelType>) {
        self.channel_types = Some(channel_types);
    }

    pub fn add_choice(&mut self, name: impl Into<String>, value: impl Into<CommandOptionChoiceValue>) -> &mut Self {
        let choice = CommandOptionChoice {
            name: name.into(),
            value: value.into(),
            name_localizations: None,
        };
        self.choices.get_or_insert_with(Vec::new).push(choice);
        self
    }
}

impl From<CommandOption> for TwilightCommandOption {
    fn from(value: CommandOption) -> Self {
        TwilightCommandOption {
            name: value.name,
            description: value.description,
            autocomplete: value.autocomplete,
            channel_types: value.channel_types,
            required: value.required,
            kind: value.kind,
            choices: None,
            name_localizations: None,
            description_localizations: None,
            options: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
        }
    }
}