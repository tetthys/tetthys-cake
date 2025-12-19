use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use once_cell::sync::OnceCell;

use crate::engine::{Decision, DecisionKind};
use crate::integration::cake::actor::{ActorResolver, DefaultActorResolver};
use crate::integration::cake::policy::{norm_method, norm_resource, ObjectRequirement, PolicyDecl};
use crate::model::{Action, Actor, Context, IntoObjectRef, ObjectRef};

#[derive(Clone)]
pub struct CakeConfig {
    pub actor_resolver: Arc<dyn ActorResolver>,
    pub context: Context,
}

impl Default for CakeConfig {
    fn default() -> Self {
        Self {
            actor_resolver: Arc::new(DefaultActorResolver),
            context: Context::default(),
        }
    }
}

#[derive(Clone)]
pub struct CakeState {
    pub engine: crate::Engine,
    pub cfg: CakeConfig,
}

impl CakeState {
    pub fn new(cfg: CakeConfig) -> Self {
        Self { engine: crate::Engine::new(), cfg }
    }
}

#[derive(Clone)]
pub struct Cake {
    state: CakeState,
    actor: Actor,
}

type Key = (String, String); // (resource, method)

static POLICY_MAP: OnceCell<HashMap<Key, &'static PolicyDecl>> = OnceCell::new();

fn build_policy_map() -> HashMap<Key, &'static PolicyDecl> {
    let mut map: HashMap<Key, &'static PolicyDecl> = HashMap::new();

    for decl in inventory::iter::<PolicyDecl> {
        let k = (norm_resource(decl.resource), norm_method(decl.method));

        if let Some(prev) = map.get(&k) {
            // higher priority wins; tie => panic
            if decl.priority == prev.priority {
                panic!(
                    "tetthys-cake: duplicate policy key {:?} with same priority {}",
                    k, decl.priority
                );
            }

            if decl.priority > prev.priority {
                map.insert(k, decl);
            }
        } else {
            map.insert(k, decl);
        }
    }

    map
}

impl Cake {
    fn policy_map() -> &'static HashMap<Key, &'static PolicyDecl> {
        POLICY_MAP.get_or_init(build_policy_map)
    }

    fn parse_action(action: &str) -> (String, String) {
        let mut it = action.splitn(2, '.');
        let res = it.next().unwrap_or("");
        let method = it.next().unwrap_or("");
        (norm_resource(res), norm_method(method))
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }

    pub fn context(&self) -> &Context {
        &self.state.cfg.context
    }

    pub fn decide<O: IntoObjectRef>(&self, action_str: &str, object: O) -> Decision {
        let (res, method) = Self::parse_action(action_str);
        let key = (res, method);

        let Some(decl) = Self::policy_map().get(&key) else {
            return Decision { kind: DecisionKind::Deny, selected_rule: None, trace: vec![] };
        };

        let action = Action::new(action_str);
        let oref = object.into_object_ref();
        let ctx = &self.state.cfg.context;

        if decl.object_requirement == ObjectRequirement::Required && oref.is_none() {
            return Decision {
                kind: DecisionKind::Deny,
                selected_rule: None,
                trace: vec![],
            };
        }

        let rules = (decl.f)(&self.actor, &action, &oref, ctx);
        self.state.engine.decide(&self.actor, &action, &oref, ctx, &rules)
    }

    pub fn can<O: IntoObjectRef>(&self, action: &str, object: O) -> bool {
        self.decide(action, object).is_permit()
    }

    pub fn authorize<O: IntoObjectRef>(&self, action: &str, object: O) -> Result<Decision, crate::integration::cake::error::CakeError> {
        let d = self.decide(action, object);
        if d.is_permit() {
            Ok(d)
        } else {
            Err(crate::integration::cake::error::CakeError::forbidden(action, d))
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Cake
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let state = parts.extensions.get::<CakeState>().cloned().ok_or(())?;
        let actor = state.cfg.actor_resolver.from_parts(parts).await;
        Ok(Self { state, actor })
    }
}
