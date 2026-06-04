use twilight_model::{
    channel::{
        message::{
            Embed, 
            MessageFlags
        }
    }, 
    http::interaction::{
        InteractionResponse, 
        InteractionResponseData, 
        InteractionResponseType
    }
};

pub mod builder;

pub struct CommandResponse {
    content: Option<String>,
    embeds: Vec<Embed>,
    ephemeral: bool,
}

impl CommandResponse {
    pub fn new() -> Self {
        Self {
            content: None,
            embeds: vec![],
            ephemeral: false
        }
    }

    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = Some(content.into());
    }

    pub fn add_embed(&mut self, embed: Embed) {
        self.embeds.push(embed);
    }

    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.ephemeral = ephemeral;
    }
}

impl Into<InteractionResponse> for CommandResponse {
    fn into(self) -> InteractionResponse {
        InteractionResponse { 
            kind: InteractionResponseType::ChannelMessageWithSource, 
            data: Some(InteractionResponseData {
                content: self.content,
                flags: if self.ephemeral { Some(MessageFlags::EPHEMERAL) } else { None },
                embeds: if self.embeds.len() > 0 { Some(self.embeds) } else { None },
                ..Default::default()
            })
        }
    }
}