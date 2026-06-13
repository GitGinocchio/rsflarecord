use async_trait::async_trait;
use flarecord::{
    models::{ChannelType, SelectMenuType, components::{
        content::{media_gallery::{MediaGallery, MediaGalleryItem}, text_display::TextDisplay}, interactive::{button::{
            Button, 
            ButtonStyle
        }, select::Select}, layout::{
            action_row::{
                ActionRow, 
                IntoActionRow
            }, 
            container::Container, 
            section::Section, 
            separator::Separator
        }
    }}, 
    prelude::*
};


pub struct MyComponent;

#[async_trait(?Send)]
impl Component for MyComponent {
    fn id(&self) -> String {
        "mycomponent".into()
    }

    fn build(&self) -> RootComponent {
        let mut root = RootComponent::new();

        let back_button = Button::new()
            .style(ButtonStyle::Secondary)
            .label("Back")
            .build();

        let cancel_button = Button::new()
            .style(ButtonStyle::Danger)
            .label("Cancel")
            .build();

        let next_button = Button::new()
            .style(ButtonStyle::Primary)
            .label("Next")
            .build();

        let action_row = ActionRow::new()
            .button(back_button)
            .button(cancel_button)
            .button(next_button)
            .build();

        root.add(action_row);

        root
    }

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::new())
    }
}