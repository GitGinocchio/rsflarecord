use twilight_model::channel::message::embed::EmbedAuthor as TwilightEmbedAuthor;

pub struct EmbedAuthor(TwilightEmbedAuthor);

impl EmbedAuthor {
    pub fn new(name: impl Into<String>) -> Self {
        Self(TwilightEmbedAuthor { 
            name: name.into(),
            icon_url: None,
            proxy_icon_url: None,
            url: None
        })
    }

    pub fn set_icon_url(&mut self, icon_url: Option<String>) {
        self.0.icon_url = icon_url;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.0.url = url;
    }
}

impl Into<TwilightEmbedAuthor> for EmbedAuthor {
    fn into(self) -> TwilightEmbedAuthor {
        self.0
    }
}

impl Into<EmbedAuthor> for TwilightEmbedAuthor {
    fn into(self) -> EmbedAuthor {
        EmbedAuthor(self)
    }
}