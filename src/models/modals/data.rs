use std::ops::Deref;

use twilight_model::application::interaction::modal::ModalInteractionData as TwilightModalData;




#[allow(unused)]
pub struct ModalData(TwilightModalData);

#[allow(unused)]
impl ModalData {
    // TODO: Creare dei metodi helper
}

impl From<TwilightModalData> for ModalData {
    fn from(value: TwilightModalData) -> Self {
        Self(value)
    }
}

impl Deref for ModalData {
    type Target = TwilightModalData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}