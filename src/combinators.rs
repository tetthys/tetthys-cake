// src/combinators.rs
use crate::pred::Pred;

/// English comment: OR identity is false; short-circuits on first true.
pub fn s_or(preds: Vec<Pred>) -> Pred {
    Pred::new(move |u, a, o, c| {
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
    Pred::new(move |u, a, o, c| {
        for p in &preds {
            if !p.call(u, a, o, c) {
                return false;
            }
        }
        true
    })
}

/// English comment: NOT combinator.
pub fn s_not(p: Pred) -> Pred {
    p.not()
}

/// English comment: D-combinators are identical to S-combinators; names exist for readability.
pub fn d_or(preds: Vec<Pred>) -> Pred {
    s_or(preds)
}
pub fn d_and(preds: Vec<Pred>) -> Pred {
    s_and(preds)
}
pub fn d_not(p: Pred) -> Pred {
    s_not(p)
}
