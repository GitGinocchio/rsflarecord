use twilight_model::channel::message::Embed;

use super::CommandResponse;

pub struct CommandResponseBuilder(CommandResponse);

impl CommandResponseBuilder {
    pub fn new() -> Self {
        Self(CommandResponse::new())
    }

    pub fn build(self) -> CommandResponse {
        self.0
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.set_content(content);
        self
    }

    pub fn ephemeral(mut self) -> Self {
        self.0.set_ephemeral(true);
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.0.add_embed(embed);
        self
    }
}