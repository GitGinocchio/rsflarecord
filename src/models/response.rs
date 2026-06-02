use twilight_model::channel::message::Embed;




pub struct CommandResponse {
    pub content: Option<String>,
    pub embeds: Vec<Embed>,
    pub ephemeral: bool,
}

impl CommandResponse {
    pub fn new() -> Self {
        Self {
            content: None,
            embeds: vec![],
            ephemeral: false
        }
    }
}