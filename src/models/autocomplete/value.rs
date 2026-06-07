use twilight_model::application::command::CommandOptionChoiceValue;




pub struct AutocompleteValue(CommandOptionChoiceValue);

impl AutocompleteValue {
    pub fn string(value: impl Into<String>) -> Self {
        Self(CommandOptionChoiceValue::String(value.into()))
    }

    pub fn integer(value: i64) -> Self {
        Self(CommandOptionChoiceValue::Integer(value))
    }

    pub fn number(value: f64) -> Self {
        Self(CommandOptionChoiceValue::Number(value))
    }
}

impl From<&str> for AutocompleteValue {
    fn from(value: &str) -> Self {
        AutocompleteValue::string(value)
    }
}

impl From<String> for AutocompleteValue {
    fn from(value: String) -> Self {
        AutocompleteValue::string(value)
    }
}

impl From<i64> for AutocompleteValue {
    fn from(value: i64) -> Self {
        AutocompleteValue::integer(value)
    }
}

impl From<f64> for AutocompleteValue {
    fn from(value: f64) -> Self {
        AutocompleteValue::number(value)
    }
}

impl Into<CommandOptionChoiceValue> for AutocompleteValue {
    fn into(self) -> CommandOptionChoiceValue {
        self.0
    }
}