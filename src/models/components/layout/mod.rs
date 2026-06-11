use std::collections::HashMap;

use crate::models::components::{layout::{action_row::ActionRow, container::Container, section::Section, separator::Separator}};


pub mod action_row;
pub mod container;
pub mod separator;
pub mod section;

pub struct RootComponent(HashMap<i32, LayoutComponent>);

impl RootComponent {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub (crate) fn calculate_count(&self) -> usize {
        todo!()
    }

    pub fn add<C: Into<LayoutComponent>>(&mut self, component: C) {
        let current_size = self.0.len();

        let new_id = (current_size+1) as i32;

        let mut component = component.into();
        component.set_id(new_id);

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
    pub (crate) fn set_id(&mut self, id: i32) {
        match self {
            Self::ActionRow(action_row) => action_row.set_id(id),
            Self::Container(container) => container.set_id(id),
            Self::Section(section) => section.set_id(id),
            Self::Separator(separator) => separator.set_id(id)
        };
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