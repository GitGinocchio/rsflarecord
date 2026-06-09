use crate::models::attachment::outgoing::Attachment;

#[allow(unused)]
pub struct AttachmentBuilder(Attachment);

impl AttachmentBuilder {
    pub fn new() -> Self {
        Self(Attachment::default())
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.set_description(description);
        self
    }

    pub fn content(mut self, content: impl Into<Vec<u8>>) -> Self {
        self.0.set_content(content);
        self
    }

    pub fn build(self) -> Attachment {
        self.0
    }
}

impl Into<Attachment> for AttachmentBuilder {
    fn into(self) -> Attachment {
        self.build()
    }
}