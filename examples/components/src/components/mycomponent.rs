use async_trait::async_trait;
use flarecord::{
    models::components::{
        interactive::button::{
            Button, 
            ButtonStyle
        }, 
        layout::{
            action_row::{
                ActionRow, 
                IntoActionRow
            }, 
            container::Container, 
            section::Section, 
            separator::Separator
        }
    }, 
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

        let button = Button::new()
            .style(ButtonStyle::Success)
            .on_click(|_int, _ctx| {})
            .label("test")
            .build();

        let section = Section::new();
        
        let action_row = ActionRow::new()
            .button(button)
            .build();

        let separator = Separator::new()
            .divider(true)
            .spacing(3);

        root.add(separator);
        root.add(action_row);

        root
    }

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::new())
    }
}