use std::ops::Deref;

use twilight_model::application::interaction::{
    application_command::{
        CommandData as TwilightCommandData, 
        CommandOptionValue
    }
};


pub struct CommandData(TwilightCommandData);

#[allow(unused)]
impl CommandData {
    pub fn get_option(&self, name: &str) -> Option<&CommandOptionValue> {
        // TODO: questo metodo deve ritornare un tipo creato da noi che deve essere presente in ./option
        // Sarebbe meglio un metodo che e' in grado di provare a fare il cast a quello specifico tipo
        // in caso manda errore o ritorna None se non c'e' il valore
        self.options
            .iter()
            .find(|opt| opt.name == name)
            .map(|o| &o.value)
    }

    pub (crate) fn get_subcommand_name(&self) -> Option<&str> {
        self.options.iter().find_map(|opt| match opt.value {
            CommandOptionValue::SubCommand(_) => {
                Some(opt.name.as_str())
            }
            _ => None
        })
    }

    pub (crate) fn get_subcommand_group_name(&self) -> Option<&str> {
        self.options.iter().find_map(|opt| match opt.value {
            CommandOptionValue::SubCommandGroup(_) => {
                Some(opt.name.as_str())
            }
            _ => None,
        })
    }

    pub (crate) fn get_inner(&self) -> Option<CommandData> {
        self.0.options.iter().find_map(|opt| {
            if let CommandOptionValue::SubCommand(sub_options) = &opt.value {
                Some(CommandData(TwilightCommandData {
                    name: opt.name.clone(),
                    options: sub_options.clone(),
                    resolved: self.0.resolved.clone(),
                    guild_id: self.0.guild_id,
                    id: self.id,
                    kind: self.kind,
                    target_id: self.target_id
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

impl Deref for CommandData {
    type Target = TwilightCommandData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}