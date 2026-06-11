use std::marker::PhantomData;

use crate::models::components::{interactive::{button::Button, select::Select}};

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

impl ActionRow {
    pub fn new() -> ActionRowState<Empty> {
        ActionRowState { 
            components: Vec::new(), 
            _marker: PhantomData,
            id: 0
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
    components: Vec<ActionRowChild>,
    id: i32,
    _marker: PhantomData<S>,
}

impl ActionRowState<Empty> {
    pub fn select(mut self, s: Select) -> ActionRowState<HasSelect> {
        self.components.push(ActionRowChild::Select(s));
        ActionRowState { components: self.components, _marker: PhantomData, id: 0 }
    }

    pub fn button(mut self, b: Button) -> ActionRowState<Has1> {
        self.components.push(ActionRowChild::Button(b));
        ActionRowState { components: self.components, _marker: PhantomData, id: 0 }
    }
}

impl ActionRowState<Has1> {
    pub fn button(mut self, b: Button) -> ActionRowState<Has2> {
        self.components.push(ActionRowChild::Button(b));
        ActionRowState { components: self.components, _marker: PhantomData, id: 0 }
    }
}

impl ActionRowState<Has2> {
    pub fn button(mut self, b: Button) -> ActionRowState<Has3> {
        self.components.push(ActionRowChild::Button(b));
        ActionRowState { components: self.components, _marker: PhantomData, id: 0 }
    }
}

impl ActionRowState<Has3> {
    pub fn button(mut self, b: Button) -> ActionRowState<Has4> {
        self.components.push(ActionRowChild::Button(b));
        ActionRowState { components: self.components, _marker: PhantomData, id: 0 }
    }
}

impl ActionRowState<Has4> {
    pub fn button(mut self, b: Button) -> ActionRowState<Has5> {
        self.components.push(ActionRowChild::Button(b));
        ActionRowState { components: self.components, _marker: PhantomData, id: 0 }
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