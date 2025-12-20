// pred.rs
use crate::model::{Action, Actor, Context, ObjectRef};
use std::sync::Arc;

pub type PredFn =
    dyn Fn(&Actor, &Action, &ObjectRef, &Context) -> bool + Send + Sync + 'static;

#[derive(Clone)]
pub struct Pred {
    f: Arc<PredFn>,
}

impl Pred {
    pub fn s<F>(f: F) -> Self
    where
        F: Fn(&Actor, &Action, &ObjectRef, &Context) -> bool + Send + Sync + 'static,
    {
        Self { f: Arc::new(f) }
    }

    pub fn d<F>(f: F) -> Self
    where
        F: Fn(&Actor, &Action, &ObjectRef, &Context) -> bool + Send + Sync + 'static,
    {
        Self { f: Arc::new(f) }
    }

    pub fn call(&self, u: &Actor, a: &Action, o: &ObjectRef, c: &Context) -> bool {
        (self.f)(u, a, o, c)
    }
}
