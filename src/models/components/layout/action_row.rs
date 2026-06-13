use std::{collections::HashMap, marker::PhantomData};

use twilight_model::{
    channel::message::{
        Component as TwilightComponent, 
        component::ActionRow as TwilightActionRow
    }
};

use crate::{models::components::{interactive::{button::Button, select::Select}}, traits::component::IntoTwilight};

pub struct Empty;
pub struct Has1;
pub struct Has2;
pub struct Has3;
pub struct Has4;
pub struct Has5;
pub struct HasSelect;

pub enum ActionRow {
    Empty(ActionRowState<Empty>),
    HasSelect(ActionRowState<HasSelect>),
    Has1(ActionRowState<Has1>),
    Has2(ActionRowState<Has2>),
    Has3(ActionRowState<Has3>),
    Has4(ActionRowState<Has4>),
    Has5(ActionRowState<Has5>),
}

pub enum ActionRowChild {
    Button(Button),
    Select(Select)
}

#[allow(unused)]
impl ActionRow {
    pub fn new() -> ActionRowState<Empty> {
        ActionRowState { 
            components: HashMap::new(), 
            _marker: PhantomData,
            id: 0
        }
    }

    pub (crate) fn get_children(&self) -> &HashMap<i32, ActionRowChild> {
        match self {
            ActionRow::Empty(a) => &a.components,
            ActionRow::Has1(a) => &a.components,
            ActionRow::Has2(a) => &a.components,
            ActionRow::Has3(a) => &a.components,
            ActionRow::Has4(a) => &a.components,
            ActionRow::Has5(a) => &a.components,
            ActionRow::HasSelect(a) => &a.components
        }
    }

    pub (crate) fn set_id(&mut self, id: i32) {
        match self {
            Self::Empty(state) => state.set_id(id),
            Self::Has1(state) => state.set_id(id),
            Self::Has2(state) => state.set_id(id),
            Self::Has3(state) => state.set_id(id),
            Self::Has4(state) => state.set_id(id),
            Self::Has5(state) => state.set_id(id),
            Self::HasSelect(state) => state.set_id(id),
        }
    }
}

pub struct ActionRowState<S> {
    pub (crate) components: HashMap<i32, ActionRowChild>,
    id: i32,
    _marker: PhantomData<S>,
}

fn add_button<T, N>(mut ars: ActionRowState<T>, mut b: Button) -> ActionRowState<N> {
    let id = (ars.components.len() + 1) as i32;
    b.set_id(format!("{}:{}", ars.id, id));
    ars.components.insert(id, ActionRowChild::Button(b));
    ActionRowState { components: ars.components, _marker: PhantomData, id: 0 }
}

impl ActionRowState<Empty> {
    pub fn select(mut self, mut s: Select) -> ActionRowState<HasSelect> {
        // NOTE: qui mettiamo sempre l'indice 0 visto che e' possibile avere solo un select!
        s.set_id(format!("{}:{}", self.id, 0));
        self.components.insert(0, ActionRowChild::Select(s));
        ActionRowState { components: self.components, _marker: PhantomData, id: 0 }
    }

    pub fn button(self, b: Button) -> ActionRowState<Has1> {
        add_button(self, b)
    }
}

impl ActionRowState<Has1> {
    pub fn button(self, b: Button) -> ActionRowState<Has2> {
        add_button(self, b)
    }
}

impl ActionRowState<Has2> {
    pub fn button(self, b: Button) -> ActionRowState<Has3> {
        add_button(self, b)
    }
}

impl ActionRowState<Has3> {
    pub fn button(self, b: Button) -> ActionRowState<Has4> {
        add_button(self, b)
    }
}

impl ActionRowState<Has4> {
    pub fn button(self, b: Button) -> ActionRowState<Has5> {
        add_button(self, b)
    }
}

pub trait IntoActionRow {
    fn build(self) -> ActionRow;
}

pub (crate) trait SetActionRowId {
    fn set_id(&mut self, id: i32);
}

macro_rules! impl_action_row {
    ($(($state:ident, $variant:ident)),* $(,)?) => {
        $(
            impl SetActionRowId for ActionRowState<$state> {
                fn set_id(&mut self, id: i32) {
                    self.id = id;
                }
            }

            impl IntoActionRow for ActionRowState<$state> {
                fn build(self) -> ActionRow {
                    ActionRow::$variant(self)
                }
            }

            impl Into<ActionRow> for ActionRowState<$state> {
                fn into(self) -> ActionRow {
                    ActionRow::$variant(self)
                }
            }
        )*
    };
}

impl_action_row!(
    (Empty, Empty),
    (Has1, Has1),
    (Has2, Has2),
    (Has3, Has3),
    (Has4, Has4),
    (Has5, Has5),
    (HasSelect, HasSelect),
);

impl IntoTwilight<TwilightComponent> for ActionRowChild {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Button(button) => TwilightComponent::Button(button.into_twilight()),
            Self::Select(select) => TwilightComponent::SelectMenu(select.into_twilight())
        }
    }
}

impl<T> IntoTwilight<TwilightActionRow> for ActionRowState<T> {
    fn into_twilight(self) -> TwilightActionRow {
        TwilightActionRow {
            id: Some(self.id),
            components: self.components.into_iter().map(|(_id, c)| c.into_twilight()).collect()
        }
    }
}

impl IntoTwilight<TwilightComponent> for ActionRow {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::Empty(empty) => TwilightComponent::ActionRow(empty.into_twilight()),
            Self::Has1(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has2(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has3(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has4(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::Has5(select) => TwilightComponent::ActionRow(select.into_twilight()),
            Self::HasSelect(select) => TwilightComponent::ActionRow(select.into_twilight()),
        }
    }
}