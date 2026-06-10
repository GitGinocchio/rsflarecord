use twilight_model::channel::message::embed::EmbedThumbnail as TwilightEmbedThumbnail;


pub struct EmbedThumbnail(TwilightEmbedThumbnail);

impl EmbedThumbnail {
    pub fn new(url: impl Into<String>) -> Self {
        Self(TwilightEmbedThumbnail {
            height: None,
            width: None,
            proxy_url: None,
            url: url.into()
        })
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        self.0.url = url.into()
    }

    pub fn set_height(&mut self, height: Option<u64>) {
        self.0.height = height;
    }

    pub fn set_width(&mut self, width: Option<u64>) {
        self.0.height = width;
    }
}

impl From<&str> for EmbedThumbnail {
    fn from(value: &str) -> Self {
        EmbedThumbnail::new(value)
    }
}

impl From<String> for EmbedThumbnail {
    fn from(value: String) -> Self {
        EmbedThumbnail::new(value)
    }
}

impl From<TwilightEmbedThumbnail> for EmbedThumbnail {
    fn from(value: TwilightEmbedThumbnail) -> Self {
        Self(value)
    }
}

impl Into<TwilightEmbedThumbnail> for EmbedThumbnail {
    fn into(self) -> TwilightEmbedThumbnail {
        self.0
    }
}