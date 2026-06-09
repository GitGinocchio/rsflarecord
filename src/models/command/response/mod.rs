use twilight_model::{
    channel::message::{
            Embed, 
            MessageFlags
        }, 
    http::{
        attachment::Attachment as TwilightOutgoingAttachment, 
        interaction::{
            InteractionResponse, 
            InteractionResponseData, 
            InteractionResponseType
        }
    }
};

use crate::models::attachment::outgoing::Attachment;

pub mod builder;

pub struct CommandResponse {
    content: Option<String>,
    embeds: Vec<Embed>,
    attachments: Vec<Attachment>,
    ephemeral: bool,
}

impl CommandResponse {
    pub fn new() -> Self {
        Self {
            content: None,
            embeds: vec![],
            attachments: vec![],
            ephemeral: false
        }
    }

    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = Some(content.into());
    }

    pub fn add_embed(&mut self, embed: Embed) {
        self.embeds.push(embed);
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment)
    }

    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.ephemeral = ephemeral;
    }
}

impl Into<InteractionResponse> for CommandResponse {
    fn into(self) -> InteractionResponse {
        let attachments: Vec<TwilightOutgoingAttachment> = self.attachments
            .into_iter()
            .enumerate()
            .map(|(i, mut file)| {
                file.set_id(i as u64);
                file.into()
            })
            .collect();

        InteractionResponse { 
            kind: InteractionResponseType::ChannelMessageWithSource, 
            data: Some(InteractionResponseData {
                content: self.content,
                flags: if self.ephemeral { Some(MessageFlags::EPHEMERAL) } else { None },
                embeds: if self.embeds.len() > 0 { Some(self.embeds) } else { None },
                attachments: if attachments.len() > 0 { Some(attachments) } else { None },
                ..Default::default()
            })
        }
    }
}