use std::collections::HashMap;

use crate::models::components::layout::{action_row::ActionRow, section::Section, separator::Separator};

pub enum ContainerChild {
    ActionRow(ActionRow),
    Section(Section),
    Separator(Separator)
}

impl ContainerChild {
    pub (crate) fn set_id(&mut self, id: i32) {
        match self {
            Self::ActionRow(action_row) => action_row.set_id(id),
            Self::Section(section) => section.set_id(id),
            Self::Separator(separator) => separator.set_id(id)
        }
    }
}

impl From<Section> for ContainerChild {
    fn from(value: Section) -> Self {
        Self::Section(value)
    }
}

impl From<ActionRow> for ContainerChild {
    fn from(value: ActionRow) -> Self {
        Self::ActionRow(value)
    }
}

impl From<Separator> for ContainerChild {
    fn from(value: Separator) -> Self {
        Self::Separator(value)
    }
}

pub struct Container {
    children: HashMap<i32, ContainerChild>,
    id: i32
}

impl Container {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            id: 0
        }
    }

    pub (crate) fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn add(mut self, mut child: ContainerChild) -> Self {
        let current_size = self.children.len();

        let new_id = (current_size+1) as i32;
        child.set_id(new_id);

        self.children.insert(new_id, child);
        self
    }
}