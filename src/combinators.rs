// combinators.rs
use crate::model::{Action, Actor, Context, ObjectRef};
use crate::pred::Pred;

pub struct Combinators;

impl Combinators {
    /// English comment: OR identity is false; short-circuits on first true.
    pub fn s_or(preds: Vec<Pred>) -> Pred {
        Pred::s(move |u, a, o, c| {
            for p in &preds {
                if p.call(u, a, o, c) {
                    return true;
                }
            }
            false
        })
    }

    /// English comment: AND identity is true; short-circuits on first false.
    pub fn s_and(preds: Vec<Pred>) -> Pred {
        Pred::s(move |u, a, o, c| {
            for p in &preds {
                if !p.call(u, a, o, c) {
                    return false;
                }
            }
            true
        })
    }

    pub fn s_not(p: Pred) -> Pred {
        Pred::s(move |u, a, o, c| !p.call(u, a, o, c))
    }

    pub fn d_or(preds: Vec<Pred>) -> Pred {
        Pred::d(move |u, a, o, c| {
            for p in &preds {
                if p.call(u, a, o, c) {
                    return true;
                }
            }
            false
        })
    }

    pub fn d_and(preds: Vec<Pred>) -> Pred {
        Pred::d(move |u, a, o, c| {
            for p in &preds {
                if !p.call(u, a, o, c) {
                    return false;
                }
            }
            true
        })
    }

    pub fn d_not(p: Pred) -> Pred {
        Pred::d(move |u, a, o, c| !p.call(u, a, o, c))
    }
}
