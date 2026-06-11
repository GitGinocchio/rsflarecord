use twilight_model::{application::{interaction::{InteractionContextType, InteractionData, InteractionPartialGuild}, monetization::Entitlement}, channel::Channel, guild::{PartialMember, Permissions}, id::{AnonymizableId, Id, marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker}}, oauth::ApplicationIntegrationMap};

use crate::{error::Error, models::{interaction::Interaction, modals::data::ModalData, user::{User, UserRef}}};

#[allow(unused)]
pub struct ModalInteraction {
    pub data: ModalData, 
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

#[allow(unused)]
impl ModalInteraction {
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

impl TryFrom<Interaction> for ModalInteraction {
    type Error = Error;

    fn try_from(mut value: Interaction) -> Result<Self, Self::Error> {
        let data = match value.data.take() {
            Some(InteractionData::ModalSubmit(d)) => ModalData::from(*d),
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