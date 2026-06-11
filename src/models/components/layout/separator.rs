use twilight_model::channel::message::component::Separator as TwilightSeparator;




#[derive(Clone)]
pub struct Separator(TwilightSeparator);

impl Separator {
    pub fn new() -> Self {
        Self(TwilightSeparator {
            id: None,
            divider: None,
            spacing: None
        })
    }

    pub (crate) fn set_id(&mut self, id: i32) {
        self.0.id = Some(id);
    }

    pub fn spacing(mut self, spacing: u8) -> Self {
        self.0.spacing = Some(spacing.into());
        self
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.0.divider = Some(divider);
        self
    }
}