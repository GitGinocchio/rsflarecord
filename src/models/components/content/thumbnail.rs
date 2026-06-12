use twilight_model::channel::message::component::{Thumbnail as TwilightThumbnail, UnfurledMediaItem};

use crate::traits::component::IntoTwilight;





pub struct Thumbnail(TwilightThumbnail);

impl Thumbnail {
    pub fn new(url: impl Into<String>) -> Self {
        Self(TwilightThumbnail {
            id: None,
            media: UnfurledMediaItem {
                url: url.into(),
                proxy_url: None,
                height: None,
                width: None,
                content_type: None
            },
            description: None,
            spoiler: None
        })
    }
}

impl IntoTwilight<TwilightThumbnail> for Thumbnail {
    fn into_twilight(self) -> TwilightThumbnail {
        self.0
    }
}