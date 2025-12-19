use crate::model::{Action, Actor, Context, ObjectRef};
use crate::rule::RuleSet;

pub type PolicyFn = fn(&Actor, &Action, &ObjectRef, &Context) -> RuleSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectRequirement {
    Optional,
    Required,
}

pub struct PolicyDecl {
    pub resource: &'static str, // normalized
    pub method: &'static str,   // normalized
    pub priority: i32,
    pub object_requirement: ObjectRequirement,
    pub f: PolicyFn,
}

inventory::collect!(PolicyDecl);

pub fn norm_resource(s: &str) -> String {
    s.chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

pub fn norm_method(s: &str) -> String {
    s.chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}
