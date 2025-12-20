// src/lib.rs
pub mod combinators;
pub mod engine;
pub mod model;
pub mod pred;
pub mod rule;

pub use combinators::*;
pub use engine::{Decision, DecisionKind, Engine, TraceEvent, TraceResult};
pub use model::{Action, Actor, Context, IntoObjectRef, ObjectRef};
pub use pred::{Pred, PredFn};
pub use rule::{Rule, RuleSet, RuleSetMode};

/// English comment: Build subject predicate with move-capture and arity adaptation.
#[macro_export]
macro_rules! pred_s {
    (|$u:ident| $body:expr) => {
        $crate::Pred::new(move |$u, _a, _o, _c| $body)
    };
    (|$u:ident, $a:ident, $o:ident| $body:expr) => {
        $crate::Pred::new(move |$u, $a, $o, _c| $body)
    };
    (|$u:ident, $a:ident, $o:ident, $c:ident| $body:expr) => {
        $crate::Pred::new(move |$u, $a, $o, $c| $body)
    };
}

/// English comment: Build domain predicate with move-capture and arity adaptation.
#[macro_export]
macro_rules! pred_d {
    (|$u:ident| $body:expr) => {
        $crate::Pred::new(move |$u, _a, _o, _c| $body)
    };
    (|$u:ident, $a:ident, $o:ident| $body:expr) => {
        $crate::Pred::new(move |$u, $a, $o, _c| $body)
    };
    (|$u:ident, $a:ident, $o:ident, $c:ident| $body:expr) => {
        $crate::Pred::new(move |$u, $a, $o, $c| $body)
    };
}

/// English comment: Rule builder macro (PHP-like).
#[macro_export]
macro_rules! rule {
    ($name:expr, $s:expr, $d:expr) => {
        $crate::Rule::new($name, $s, $d)
    };
}

/// English comment: RuleSet builders (PHP-like allOf/anyOf).
#[macro_export]
macro_rules! rules_all {
    ( $( $rule:expr ),* $(,)? ) => {
        $crate::RuleSet::all_of(vec![ $( $rule ),* ])
    };
}

#[macro_export]
macro_rules! rules_any {
    ( $( $rule:expr ),* $(,)? ) => {
        $crate::RuleSet::any_of(vec![ $( $rule ),* ])
    };
}

/// English comment: Combinator macros (avoid vec! noise).
#[macro_export]
macro_rules! s_or {
    ( $( $p:expr ),* $(,)? ) => {
        $crate::combinators::s_or(vec![ $( $p ),* ])
    };
}
#[macro_export]
macro_rules! s_and {
    ( $( $p:expr ),* $(,)? ) => {
        $crate::combinators::s_and(vec![ $( $p ),* ])
    };
}
#[macro_export]
macro_rules! s_not {
    ($p:expr) => {
        $crate::combinators::s_not($p)
    };
}
#[macro_export]
macro_rules! d_or {
    ( $( $p:expr ),* $(,)? ) => {
        $crate::combinators::d_or(vec![ $( $p ),* ])
    };
}
#[macro_export]
macro_rules! d_and {
    ( $( $p:expr ),* $(,)? ) => {
        $crate::combinators::d_and(vec![ $( $p ),* ])
    };
}
#[macro_export]
macro_rules! d_not {
    ($p:expr) => {
        $crate::combinators::d_not($p)
    };
}
