use twilight_model::channel::message::Embed;

use crate::models::attachment::outgoing::Attachment;

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

    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        for attachment in attachments {
            self.0.add_attachment(attachment);
        }
        self
    }

    pub fn attachment(mut self, attachment: Attachment) -> Self {
        self.0.add_attachment(attachment);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        for embed in embeds {
            self.0.add_embed(embed);
        }
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.0.add_embed(embed);
        self
    }
}