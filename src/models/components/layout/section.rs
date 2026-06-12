use std::marker::PhantomData;

use twilight_model::channel::message::{
    Component as TwilightComponent, 
    component::Section as TwilightSection
};

use crate::{models::components::{content::{text_display::TextDisplay, thumbnail::Thumbnail}, interactive::button::Button}, traits::component::IntoTwilight};




pub enum SectionComponent {
    TextDisplay(TextDisplay)
}

pub enum SectionAccessory {
    Button(Button),
    Thumbnail(Thumbnail)
}

pub struct Empty;
pub struct HasComponent;
pub struct HasAccessory;
pub struct Ready;

pub enum Section {
    Ready(SectionState<HasComponent, HasAccessory>),
}

impl Section {
    pub fn new() -> SectionState<Empty, Empty> {
        SectionState::new()
    }

    pub (crate) fn set_id(&mut self, id: i32) {
        match self {
            Self::Ready(state) => state.id = Some(id)
        }
    }
}

#[allow(unused)]
pub struct SectionState<C, A> {
    id: Option<i32>,
    components: Vec<SectionComponent>,
    accessory: Option<SectionAccessory>,
    _marker: PhantomData<(C, A)>
}

impl SectionState<Empty, Empty> {
    fn new() -> Self {
        Self {
            id: None,
            components: Vec::new(),
            accessory: None,
            _marker: PhantomData
        }
    }

    pub fn accessory(self, accessory: SectionAccessory) -> SectionState<Empty, HasAccessory> {
        SectionState {
            id: self.id,
            components: self.components,
            accessory: Some(accessory),
            _marker: PhantomData
        }
    }

    pub fn component(mut self, component: SectionComponent) -> SectionState<HasComponent, Empty> {
        self.components.push(component);
        SectionState {
            id: self.id,
            components: self.components,
            accessory: None,
            _marker: PhantomData
        }
    }
}

impl SectionState<HasComponent, Empty> {
    pub fn component(mut self, component: SectionComponent) -> SectionState<HasComponent, Empty> {
        self.components.push(component);
        SectionState {
            id: self.id,
            components: self.components,
            accessory: None,
            _marker: PhantomData
        }
    }
    
    pub fn accessory(self, accessory: SectionAccessory) -> SectionState<HasComponent, HasAccessory> {
        SectionState {
            id: self.id,
            components: self.components,
            accessory: Some(accessory),
            _marker: PhantomData
        }
    }
}

impl SectionState<HasComponent, HasAccessory> {
    pub fn component(mut self, component: SectionComponent) -> SectionState<HasComponent, Empty> {
        self.components.push(component);
        SectionState {
            id: self.id,
            components: self.components,
            accessory: None,
            _marker: PhantomData
        }
    }
}

impl SectionState<HasComponent, HasAccessory> {
    pub fn build(self) -> Section {
        Section::Ready(self)
    }
}

impl IntoTwilight<TwilightComponent> for SectionAccessory {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Button(button) => TwilightComponent::Button(button.into_twilight()),
            Self::Thumbnail(thumbnail) => TwilightComponent::Thumbnail(thumbnail.into_twilight())
        }
    }
}

impl IntoTwilight<TwilightComponent> for SectionComponent {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::TextDisplay(text_display) => TwilightComponent::TextDisplay(text_display.into_twilight())
        }
    }
}

impl IntoTwilight<TwilightSection> for SectionState<HasComponent, HasAccessory> {
    fn into_twilight(self) -> TwilightSection {
        TwilightSection {
            id: self.id,
            components: self.components.into_iter().map(|c| c.into_twilight()).collect(),
            accessory: Box::new(self.accessory.expect("Section should be ready").into_twilight())
        }
    }
}

impl IntoTwilight<TwilightComponent> for Section {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Ready(ready) => TwilightComponent::Section(ready.into_twilight())
        }
    }
}