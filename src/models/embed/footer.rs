use twilight_model::channel::message::embed::EmbedFooter as TwilightEmbedFooter;


pub struct EmbedFooter(TwilightEmbedFooter);

impl EmbedFooter {
    pub fn new(text: impl Into<String>) -> Self {
        Self(TwilightEmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: text.into()
        })
    }

    pub fn set_icon_url(&mut self, icon_url: Option<String>) {
        self.0.icon_url = icon_url
    }
}

impl Into<TwilightEmbedFooter> for EmbedFooter {
    fn into(self) -> TwilightEmbedFooter {
        self.0
    }
}

impl From<TwilightEmbedFooter> for EmbedFooter {
    fn from(value: TwilightEmbedFooter) -> Self {
        Self(value)
    }
}