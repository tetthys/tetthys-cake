use async_trait::async_trait;
use axum::http::request::Parts;

use crate::model::Actor;

#[async_trait]
pub trait ActorResolver: Send + Sync + 'static {
    async fn from_parts(&self, parts: &mut Parts) -> Actor;
}

#[derive(Clone, Default)]
pub struct DefaultActorResolver;

#[async_trait]
impl ActorResolver for DefaultActorResolver {
    async fn from_parts(&self, _parts: &mut Parts) -> Actor {
        Actor::new("anonymous", Vec::<String>::new())
    }
}
