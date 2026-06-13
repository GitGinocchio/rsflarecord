use std::collections::HashMap;

use twilight_model::channel::message::{
    Component as TwilightComponent, 
    component::Container as TwilightContainer
};

use crate::{models::{color::Color, components::{content::media_gallery::MediaGallery, layout::{action_row::ActionRow, section::Section, separator::Separator}}}, traits::component::IntoTwilight};

pub enum ContainerChild {
    ActionRow(ActionRow),
    Section(Section),
    Separator(Separator),
    MediaGallery(MediaGallery)
}

impl ContainerChild {
    pub (crate) fn set_id(&mut self,component_id: &str, id: i32) {
        match self {
            Self::ActionRow(action_row) => action_row.set_id(component_id, id),
            Self::Section(section) => section.set_id(component_id, id),
            Self::Separator(separator) => separator.set_id(component_id, id),
            Self::MediaGallery(mg) => mg.set_id(id)
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

impl From<MediaGallery> for ContainerChild {
    fn from(value: MediaGallery) -> Self {
        Self::MediaGallery(value)
    }
}

pub struct Container {
    children: HashMap<i32, ContainerChild>,
    accent_color: Option<Color>,
    spoiler: Option<bool>,
    id: i32
}

impl Container {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            accent_color: None,
            spoiler: None,
            id: 0
        }
    }

    pub (crate) fn set_id(&mut self, component_id: &str, id: i32) {
        self.id = id;

        for (id, child) in &mut self.children {
            child.set_id(component_id, *id);
        }
    }

    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn add(mut self, child: impl Into<ContainerChild>) -> Self {
        let id = (self.children.len()+1) as i32;
        let child = child.into();

        self.children.insert(id, child);
        self
    }
}

impl IntoTwilight<TwilightComponent> for ContainerChild {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::ActionRow(action_row) => action_row.into_twilight(),
            Self::Section(section) => section.into_twilight(),
            Self::Separator(separator) => separator.into_twilight(),
            Self::MediaGallery(media_gallery) => media_gallery.into_twilight()
        }
    }
}

impl IntoTwilight<TwilightContainer> for Container {
    fn into_twilight(self) -> TwilightContainer {
        let mut children: Vec<_> = self.children.into_iter().collect();
        children.sort_by_key(|(id, _c)| *id);

        TwilightContainer {
            id: Some(self.id),
            spoiler: self.spoiler,
            accent_color: Some(self.accent_color.map(|v| v.into())),
            components: children
                .into_iter()
                .map(|(_id, c)| c.into_twilight())
                .collect()
        }
    }
}

impl IntoTwilight<TwilightComponent> for Container {
    fn into_twilight(self) -> TwilightComponent {
        TwilightComponent::Container(self.into_twilight())
    }
}