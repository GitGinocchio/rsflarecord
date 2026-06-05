use async_trait::async_trait;
use twilight_model::id::{Id, marker::UserMarker};
use twilight_model::user::User as TwilightUser;

use crate::traits::resolvable::Resolvable;
use crate::error::Result;

#[allow(unused)]
pub struct User(TwilightUser);

impl User {

}

#[async_trait]
impl Resolvable<()> for Id<UserMarker> {
    async fn resolve(&self) -> Result<()> {
        Ok(())
    }
}