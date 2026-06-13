use std::{collections::HashMap, marker::PhantomData};

use twilight_model::channel::message::{
    Component as TwilightComponent, 
    component::Section as TwilightSection
};

use crate::{models::components::{content::{text_display::TextDisplay, thumbnail::Thumbnail}, interactive::button::Button}, traits::component::IntoTwilight};

pub enum SectionComponent {
    TextDisplay(TextDisplay)
}

impl SectionComponent {
    pub (crate) fn set_id(&mut self, id: i32) {
        match self {
            SectionComponent::TextDisplay(text_display) => text_display.set_id(id)
        }
    }
}

pub enum SectionAccessory {
    Button(Button),
    Thumbnail(Thumbnail)
}

impl SectionAccessory {
    pub (crate) fn set_id(&mut self, id: i32) {
        match self {
            SectionAccessory::Button(button) => button.set_id(id.to_string()),
            SectionAccessory::Thumbnail(thumbnail) => thumbnail.set_id(id)
        }
    }
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
    components: HashMap<i32, SectionComponent>,
    accessory: Option<SectionAccessory>,
    _marker: PhantomData<(C, A)>
}

impl SectionState<Empty, Empty> {
    fn new() -> Self {
        Self {
            id: None,
            components: HashMap::new(),
            accessory: None,
            _marker: PhantomData
        }
    }

    pub fn accessory(self, mut accessory: SectionAccessory) -> SectionState<Empty, HasAccessory> {
        // NOTE: sempre 0 perche' e' uno solo!
        accessory.set_id(0);
        SectionState {
            id: self.id,
            components: self.components,
            accessory: Some(accessory),
            _marker: PhantomData
        }
    }

    pub fn component(self, component: SectionComponent) -> SectionState<HasComponent, Empty> {
        add_component(self, component)
    }
}

fn add_component<A, B, C, D>(mut state: SectionState<A, B>, mut component: SectionComponent) -> SectionState<C, D> {
    let id = (state.components.len() + 1) as i32;
    component.set_id(id);
    state.components.insert(id, component);
    SectionState { id: state.id, components: state.components, accessory: None, _marker: PhantomData }
}

impl SectionState<HasComponent, Empty> {
    pub fn component(self, component: SectionComponent) -> SectionState<HasComponent, Empty> {
        add_component(self, component)
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
    pub fn component(self, component: SectionComponent) -> SectionState<HasComponent, Empty> {
        add_component(self, component)
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
            components: self.components.into_iter().map(|(_id, c)| c.into_twilight()).collect(),
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