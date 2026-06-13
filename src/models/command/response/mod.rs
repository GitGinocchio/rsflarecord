use twilight_model::{
    channel::message::{
        Component as TwilightComponent, 
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

use crate::{bot::Bot, models::{attachment::outgoing::Attachment, command::response::builder::CommandResponseBuilder, components::{Component, ComponentType}}, traits::component::{IntoComponent, IntoTwilight}};

pub mod builder;

pub struct CommandResponse {
    content: Option<String>,
    embeds: Vec<Embed>,
    attachments: Vec<Attachment>,
    components: Vec<TwilightComponent>,
    ephemeral: bool,
    require_components_v2: bool
}

impl CommandResponse {
    pub fn new() -> Self {
        Self {
            content: None,
            embeds: vec![],
            attachments: vec![],
            components: vec![],
            ephemeral: false,
            require_components_v2: false
        }
    }

    pub fn builder() -> CommandResponseBuilder {
        CommandResponseBuilder::new()
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

    /// Adds a component to the response.
    ///
    /// The `component` argument can be any type that implements [`IntoComponent`].
    /// 
    /// This includes:
    /// - **Layout Components**: Any type that can be converted into a [`LayoutComponent`], 
    ///   specifically: [`ActionRow`], [`Container`], [`Section`], or [`Separator`].
    /// - **Custom Components**: Any type implementing the [`Component`] trait.
    pub fn add_component(&mut self, component: impl IntoComponent) {
        match component.into_component() {
            ComponentType::Base(component) => {
                if component.require_components_v2() {
                    self.require_components_v2 = true;
                }

                self.components.push(component.into_twilight())
            },
            ComponentType::Custom(custom) => {
                let built_component = custom.build();

                if built_component.require_components_v2() {
                    self.require_components_v2 = true;
                }

                let mut components: Vec<TwilightComponent> = built_component.into_twilight();
                worker::console_debug!("twilight_component: {components:?}");
                self.components.append(&mut components);
            }
        }
    }

    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.ephemeral = ephemeral;
    }
}

impl IntoTwilight<InteractionResponse> for CommandResponse {
    fn into_twilight(self) -> InteractionResponse {
        let attachments: Vec<TwilightOutgoingAttachment> = self.attachments
            .into_iter()
            .enumerate()
            .map(|(i, mut file)| {
                file.set_id(i as u64);
                file.into()
            })
            .collect();

        let mut flags = MessageFlags::empty();

        if self.ephemeral {
            flags = flags.union(MessageFlags::EPHEMERAL);
        }

        if self.require_components_v2 {
            flags = flags.union(MessageFlags::IS_COMPONENTS_V2)
        }

        InteractionResponse { 
            kind: InteractionResponseType::ChannelMessageWithSource, 
            data: Some(InteractionResponseData {
                content: self.content,
                flags: Some(flags),
                embeds: if self.embeds.len() > 0 { Some(self.embeds) } else { None },
                attachments: if attachments.len() > 0 { Some(attachments) } else { None },
                components: if self.components.len() > 0 { Some(self.components) } else { None },
                ..Default::default()
            })
        }
    }
}