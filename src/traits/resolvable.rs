
use async_trait::async_trait;

use crate::error::BotResult;

#[async_trait(?Send)]
pub trait Resolvable<T> {
    async fn resolve(&self) -> BotResult<T>;
}