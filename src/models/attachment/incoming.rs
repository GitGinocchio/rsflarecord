use std::ops::Deref;

use twilight_model::channel::Attachment as TwilightIncomingAttachment;

use crate::error::Error;

#[allow(unused)]
pub struct IncomingAttachment(TwilightIncomingAttachment);

#[allow(unused)]
pub struct IncomingAttachmentRef<'a>(&'a TwilightIncomingAttachment);

impl IncomingAttachment {
    pub fn is_image(&self) -> bool {
        self.content_type.as_ref().map_or(false, |ct| ct.starts_with("image/"))
    }

    pub fn is_video(&self) -> bool {
        self.content_type.as_ref().map_or(false, |ct| ct.starts_with("video/"))
    }

    pub fn is_audio(&self) -> bool {
        self.content_type.as_ref().map_or(false, |ct| ct.starts_with("audio/"))
    }

    pub fn is_landscape(&self) -> bool {
        match (self.width, self.height) {
            (Some(w), Some(h)) => w > h,
            _ => false,
        }
    }

    /// returns the size of the file in MB
    pub fn size_in_mb(&self) -> f64 {
        self.size as f64 / 1_048_576.0
    }

    pub async fn download(&self) -> Result<Vec<u8>, worker::Error> {
        let Ok(url) = self.url.parse::<worker::Url>() else {
            return Err(Error::Generic("Invalid Attachment url".into()).into())
        };

        let mut response = worker::Fetch::Url(url).send().await?;
        response.bytes().await
    }
}

impl<'a> IncomingAttachmentRef<'a> {
    pub fn is_image(&self) -> bool {
        self.content_type.as_ref().map_or(false, |ct| ct.starts_with("image/"))
    }

    pub fn is_video(&self) -> bool {
        self.content_type.as_ref().map_or(false, |ct| ct.starts_with("video/"))
    }

    pub fn is_audio(&self) -> bool {
        self.content_type.as_ref().map_or(false, |ct| ct.starts_with("audio/"))
    }

    pub fn is_landscape(&self) -> bool {
        match (self.width, self.height) {
            (Some(w), Some(h)) => w > h,
            _ => false,
        }
    }

    /// returns the size of the file in MB
    pub fn size_in_mb(&self) -> f64 {
        self.size as f64 / 1_048_576.0
    }

    pub async fn download(&self) -> Result<Vec<u8>, worker::Error> {
        let Ok(url) = self.url.parse::<worker::Url>() else {
            return Err(Error::Generic("Invalid Attachment url".into()).into())
        };

        let mut response = worker::Fetch::Url(url).send().await?;
        response.bytes().await
    }
}

impl From<TwilightIncomingAttachment> for IncomingAttachment {
    fn from(value: TwilightIncomingAttachment) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a TwilightIncomingAttachment> for IncomingAttachmentRef<'a> {
    fn from(value: &'a TwilightIncomingAttachment) -> Self {
        Self(value)
    }
}

impl Deref for IncomingAttachment {
    type Target = TwilightIncomingAttachment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Deref for IncomingAttachmentRef<'a> {
    type Target = TwilightIncomingAttachment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}