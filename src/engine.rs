use crate::model::{Action, Actor, Context, ObjectRef};
use crate::rule::{RuleSet, RuleSetMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionKind {
    Permit,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub kind: DecisionKind,
    pub selected_rule: Option<String>,
    pub trace: Vec<String>,
}

impl Decision {
    pub fn is_permit(&self) -> bool {
        self.kind == DecisionKind::Permit
    }

    pub fn is_deny(&self) -> bool {
        self.kind == DecisionKind::Deny
    }
}

#[derive(Default)]
pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn decide(
        &self,
        u: &Actor,
        a: &Action,
        o: &ObjectRef,
        c: &Context,
        ruleset: &RuleSet,
    ) -> Decision {
        if ruleset.rules.is_empty() {
            // English comment: deny-by-default (both modes).
            return Decision {
                kind: DecisionKind::Deny,
                selected_rule: None,
                trace: vec![],
            };
        }

        match ruleset.mode {
            RuleSetMode::AllMustMatch => {
                let mut trace = vec![];

                for r in &ruleset.rules {
                    if r.matches(u, a, o, c) {
                        trace.push(format!("[{}] match", r.name));
                        continue;
                    }

                    trace.push(format!("[{}] no-match", r.name));
                    // English comment: short-circuit deny on first failure.
                    return Decision {
                        kind: DecisionKind::Deny,
                        selected_rule: None,
                        trace,
                    };
                }

                Decision {
                    kind: DecisionKind::Permit,
                    selected_rule: Some("all-rules".to_string()),
                    trace,
                }
            }

            RuleSetMode::AnyMayMatch => {
                let mut trace = vec![];

                for r in &ruleset.rules {
                    if r.matches(u, a, o, c) {
                        trace.push(format!("[{}] match", r.name));
                        // English comment: short-circuit permit on first match.
                        return Decision {
                            kind: DecisionKind::Permit,
                            selected_rule: Some(r.name.clone()),
                            trace,
                        };
                    }

                    trace.push(format!("[{}] no-match", r.name));
                }

                Decision {
                    kind: DecisionKind::Deny,
                    selected_rule: None,
                    trace,
                }
            }
        }
    }
}
