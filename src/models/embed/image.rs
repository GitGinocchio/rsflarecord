use twilight_model::channel::message::embed::EmbedImage as TwilightEmbedImage;



pub struct EmbedImage(TwilightEmbedImage);

impl EmbedImage {
    pub fn new(url: impl Into<String>) -> Self {
        Self(TwilightEmbedImage {
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

impl Into<EmbedImage> for &str {
    fn into(self) -> EmbedImage {
        EmbedImage::new(self)
    }
}

impl Into<EmbedImage> for String {
    fn into(self) -> EmbedImage {
        EmbedImage::new(self)
    }
}

impl From<TwilightEmbedImage> for EmbedImage {
    fn from(value: TwilightEmbedImage) -> Self {
        Self(value)
    }
}

impl Into<TwilightEmbedImage> for EmbedImage {
    fn into(self) -> TwilightEmbedImage {
        self.0
    }
}