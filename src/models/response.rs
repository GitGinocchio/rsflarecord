use twilight_model::{channel::message::Embed, http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}};




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

impl Into<InteractionResponse> for CommandResponse {
    fn into(self) -> InteractionResponse {
        InteractionResponse { 
            kind: InteractionResponseType::ChannelMessageWithSource, 
            data: Some(InteractionResponseData {
                ..Default::default()
            })
        }
    }
}