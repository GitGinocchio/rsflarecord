
use async_trait::async_trait;

use crate::error::Result;

#[async_trait(?Send)]
pub trait Resolvable<T> {
    async fn resolve(&self) -> Result<T>;
}