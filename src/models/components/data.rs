use std::ops::Deref;

use twilight_model::application::interaction::message_component::MessageComponentInteractionData as TwilightComponentData;


pub struct ComponentData(TwilightComponentData);

impl ComponentData {

}

impl From<TwilightComponentData> for ComponentData {
    fn from(value: TwilightComponentData) -> Self {
        Self(value)
    }
}

impl Deref for ComponentData {
    type Target = TwilightComponentData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}