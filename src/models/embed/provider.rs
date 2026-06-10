use twilight_model::channel::message::embed::EmbedProvider as TwilightEmbedProvider;



pub struct EmbedProvider(TwilightEmbedProvider);

impl EmbedProvider {
    pub fn new(name: Option<String>, url: Option<String>) -> Self {
        Self(TwilightEmbedProvider { 
            name: name, 
            url: url 
        })
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.0.name = name;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.0.url = url;
    }
}

impl From<TwilightEmbedProvider> for EmbedProvider {
    fn from(value: TwilightEmbedProvider) -> Self {
        Self(value)
    }
}

impl Into<TwilightEmbedProvider> for EmbedProvider {
    fn into(self) -> TwilightEmbedProvider {
        self.0
    }
}