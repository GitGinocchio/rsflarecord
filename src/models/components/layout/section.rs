use std::marker::PhantomData;

use twilight_model::channel::message::component::Section as TwilightSection;




pub enum SectionComponent {
    TextDisplay
}

pub enum SectionAccessory {
    Button,
    Thumbnail
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

/*
impl Into<TwilightSection> for Section {
    fn into(self) -> TwilightSection {
        match self {
            Self::Ready(ready) => {
                TwilightSection {
                    id: ready.id,
                    components: Vec::new(),
                    accessory: Box::new(ready.accessory.expect("Section should be ready").into())
                }
            }
        }
    }
}
*/