use crate::model::{Action, Actor, Context, ObjectRef};
use crate::pred::Pred;

#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub s: Pred,
    pub d: Pred,
}

impl Rule {
    pub fn new(name: impl Into<String>, s: Pred, d: Pred) -> Self {
        Self {
            name: name.into(),
            s,
            d,
        }
    }

    /// English comment: Rule matches only if both S and D return true.
    pub fn matches(&self, u: &Actor, a: &Action, o: &ObjectRef, c: &Context) -> bool {
        self.s.call(u, a, o, c) && self.d.call(u, a, o, c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSetMode {
    AllMustMatch,
    AnyMayMatch,
}

#[derive(Clone)]
pub struct RuleSet {
    pub mode: RuleSetMode,
    pub rules: Vec<Rule>,
}

impl RuleSet {
    pub fn all_of(rules: Vec<Rule>) -> Self {
        Self {
            mode: RuleSetMode::AllMustMatch,
            rules,
        }
    }

    pub fn any_of(rules: Vec<Rule>) -> Self {
        Self {
            mode: RuleSetMode::AnyMayMatch,
            rules,
        }
    }
}
