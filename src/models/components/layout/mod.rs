use std::collections::HashMap;

use twilight_model::channel::message::Component as TwilightComponent;

use crate::{models::components::{ComponentType, layout::{action_row::ActionRow, container::Container, section::Section, separator::Separator}}, traits::component::{IntoComponent, IntoTwilight}};


pub mod action_row;
pub mod container;
pub mod separator;
pub mod section;

pub struct RootComponent(pub (crate) HashMap<i32, LayoutComponent>);

impl RootComponent {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub (crate) fn require_components_v2(&self) -> bool {
        for (_id, comp) in self.0.iter() {
            if comp.require_components_v2() {
                return true
            }
        }

        false
    }

    pub (crate) fn count(&self) -> usize {
        todo!()
    }

    pub (crate) fn get(&self, id: i32) -> Option<&LayoutComponent> {
        self.0.get(&id)
    }

    pub (crate) fn set_id(&mut self, component_id: String) {
        for (id, comp) in &mut self.0 {
            comp.set_id(&component_id, *id);
        }
    }

    pub fn add<C: Into<LayoutComponent>>(&mut self, component: C) {
        let current_size = self.0.len();
        let new_id = (current_size+1) as i32;

        let component = component.into();
        self.0.insert(new_id, component);
    }
}

pub enum LayoutComponent {
    ActionRow(ActionRow),
    Container(Container),
    Section(Section),
    Separator(Separator)
}

impl LayoutComponent {
    pub (crate) fn set_id(&mut self, component_id: &str, id: i32) {
        match self {
            Self::ActionRow(action_row) => action_row.set_id(component_id, id),
            Self::Container(container) => container.set_id(component_id, id),
            Self::Section(section) => section.set_id(component_id, id),
            Self::Separator(separator) => separator.set_id(component_id, id)
        };
    }

    pub (crate) fn require_components_v2(&self) -> bool {
        match self {
            Self::ActionRow(_) => false,
            Self::Container(_) => true,
            Self::Section(_) => true,
            Self::Separator(_) => true,
        }
    }
}

impl From<Container> for LayoutComponent {
    fn from(value: Container) -> Self {
        Self::Container(value)
    }
}

impl From<ActionRow> for LayoutComponent {
    fn from(value: ActionRow) -> Self {
        Self::ActionRow(value)
    }
}

impl From<Separator> for LayoutComponent {
    fn from(value: Separator) -> Self {
        Self::Separator(value)
    }
}

impl From<Section> for LayoutComponent {
    fn from(value: Section) -> Self {
        Self::Section(value)
    }
}

impl IntoComponent for ActionRow {
    fn into_component(self) -> ComponentType {
        ComponentType::Base(LayoutComponent::ActionRow(self))
    }
}

impl IntoComponent for Container {
    fn into_component(self) -> ComponentType {
        ComponentType::Base(LayoutComponent::Container(self))
    }
}

impl IntoComponent for Section {
    fn into_component(self) -> ComponentType {
        ComponentType::Base(LayoutComponent::Section(self))
    }
}

impl IntoComponent for Separator {
    fn into_component(self) -> ComponentType {
        ComponentType::Base(LayoutComponent::Separator(self))
    }
}

impl IntoTwilight<TwilightComponent> for LayoutComponent {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::ActionRow(action_row) => action_row.into_twilight(),
            Self::Container(container) => container.into_twilight(),
            Self::Section(section) => section.into_twilight(),
            Self::Separator(separator) => separator.into_twilight()
        }
    }
}

impl IntoTwilight<Vec<TwilightComponent>> for RootComponent {
    fn into_twilight(self) -> Vec<TwilightComponent> {
        let mut components: Vec<_> = self.0.into_iter().collect();
        components.sort_by_key(|(id, _c)| *id);

        components.into_iter()
            .map(|(_id, c)| c.into_twilight())
            .collect()
    }
}