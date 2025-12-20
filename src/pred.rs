// src/pred.rs
use crate::model::{Action, Actor, Context, ObjectRef};
use std::sync::Arc;

pub type PredFn =
    dyn Fn(&Actor, &Action, &ObjectRef, &Context) -> bool + Send + Sync + 'static;

#[derive(Clone)]
pub struct Pred {
    f: Arc<PredFn>,
}

impl Pred {
    /// English comment: Generic predicate constructor (S/D are semantic labels only).
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&Actor, &Action, &ObjectRef, &Context) -> bool + Send + Sync + 'static,
    {
        Self { f: Arc::new(f) }
    }

    pub fn call(&self, u: &Actor, a: &Action, o: &ObjectRef, c: &Context) -> bool {
        (self.f)(u, a, o, c)
    }

    /// English comment: Logical negation.
    pub fn not(self) -> Self {
        Pred::new(move |u, a, o, c| !self.call(u, a, o, c))
    }
}
