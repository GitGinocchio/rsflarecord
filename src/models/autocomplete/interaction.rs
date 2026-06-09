use twilight_model::{
    application::{
        interaction::{
            application_command::{
                CommandOptionValue as TwilightCommandOptionValue,
                CommandData as TwilightCommandData
            },
            InteractionContextType,
            InteractionData, 
            InteractionPartialGuild}, 
            monetization::Entitlement
        }, 
        channel::Channel, 
        guild::{
            PartialMember, 
            Permissions
        }, 
        id::{
            AnonymizableId, 
            Id,
            marker::{
                ApplicationMarker, 
                ChannelMarker, 
                GuildMarker, 
                InteractionMarker, 
                UserMarker
        }
    }, 
    oauth::ApplicationIntegrationMap
};

use crate::{error::Error, models::{autocomplete::data::AutocompleteData, interaction::Interaction, user::{User, UserRef}}};

#[allow(unused)]
pub struct AutocompleteInteraction {
    pub data: AutocompleteData, 
    pub id: Id<InteractionMarker>,
    pub locale: String,
    pub token: String,
    
    pub channel: Option<Channel>,
    pub channel_id: Option<Id<ChannelMarker>>,
    
    pub guild: Option<InteractionPartialGuild>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    
    pub member: Option<PartialMember>,
    pub user: Option<User>,
    
    pub context: Option<InteractionContextType>,

    pub entitlements: Vec<Entitlement>,
    pub app_permissions: Option<Permissions>,
    pub application_id: Id<ApplicationMarker>,
    pub authorizing_integration_owners: ApplicationIntegrationMap<AnonymizableId<GuildMarker>, Id<UserMarker>>
}

impl AutocompleteInteraction {
    pub (crate) fn with_inner_data(self) -> Option<Self> {
        let resolved = self.data.0.resolved;
        let guild_id = self.data.0.guild_id;
        let id = self.data.0.id;
        let kind = self.data.0.kind;
        let target_id = self.data.0.target_id;
        
        let new_data = self.data.0.options.into_iter().find_map(|opt| {
            match opt.value {
                TwilightCommandOptionValue::SubCommand(data) | 
                TwilightCommandOptionValue::SubCommandGroup(data) => {
                    Some(AutocompleteData(TwilightCommandData {
                        name: opt.name,
                        options: data, // Questo è già un Vec, lo spostiamo direttamente
                        resolved: resolved.clone(), // Vedi nota sotto
                        guild_id,
                        id,
                        kind,
                        target_id
                    }))
                },
                _ => None
            }
        });

        match new_data {
            Some(inner) => Some(Self { data: inner, ..self }),
            None => None,
        }
    }

    pub fn author<'a>(&'a self) -> Option<UserRef<'a>> {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref().map(|a| a.into()),
            _ => self.user.as_ref().map(|a| a.into()),
        }
    }

    pub fn author_id(&self) -> Option<Id<UserMarker>> {
        self.author().map(|a| a.id)
    }
}

impl TryFrom<Interaction> for AutocompleteInteraction {
    type Error = Error;

    fn try_from(mut value: Interaction) -> Result<Self, Self::Error> {
        let data = match value.data.take() {
            Some(InteractionData::ApplicationCommand(d)) => AutocompleteData::from(*d),
            _ => return Err(Error::Generic("Expected ApplicationCommand".into())),
        };

        Ok(Self {
            application_id: value.application_id,
            authorizing_integration_owners: value.authorizing_integration_owners.clone(),
            channel: value.channel.take(),
            context: value.context.take(),
            entitlements: std::mem::take(&mut value.entitlements),
            guild: value.guild.take(),
            guild_locale: value.guild_locale.take(),
            locale: value.locale.take().expect("Locale should be always available"),
            data: data,
            id: value.id,
            token: std::mem::take(&mut value.token),
            #[allow(deprecated)]
            channel_id: value.channel_id,
            guild_id: value.guild_id,
            member: value.member.take(),
            user: value.user.take().map(|u| u.into()),
            app_permissions: value.app_permissions
        })
    }
}