use twilight_model::channel::message::embed::EmbedVideo as TwilightEmbedVideo;


pub struct EmbedVideo(TwilightEmbedVideo);

impl EmbedVideo {
    pub fn new() -> Self {
        Self(TwilightEmbedVideo {
            height: None,
            width: None,
            proxy_url: None,
            url: None
        })
    }

    pub fn set_url<IntoString: Into<String>>(&mut self, url: impl Into<Option<String>>) {
        self.0.url = url.into().into()
    }

    pub fn set_height(&mut self, height: impl Into<Option<u64>>) {
        self.0.height = height.into()
    }

    pub fn set_width(&mut self, width: impl Into<Option<u64>>) {
        self.0.height = width.into()
    }
}

impl From<TwilightEmbedVideo> for EmbedVideo {
    fn from(value: TwilightEmbedVideo) -> Self {
        Self(value)
    }
}

impl Into<TwilightEmbedVideo> for EmbedVideo {
    fn into(self) -> TwilightEmbedVideo {
        self.0
    }
}