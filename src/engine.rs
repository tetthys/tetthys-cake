// src/engine.rs
use crate::model::{Action, Actor, Context, ObjectRef};
use crate::rule::{RuleSet, RuleSetMode};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionKind {
    Permit,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceResult {
    Match,
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    pub rule: String,
    pub result: TraceResult,
}

impl fmt::Display for TraceEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.result {
            TraceResult::Match => write!(f, "[{}] match", self.rule),
            TraceResult::NoMatch => write!(f, "[{}] no-match", self.rule),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub kind: DecisionKind,
    pub selected_rule: Option<String>,
    pub trace: Vec<TraceEvent>,
}

impl Decision {
    pub fn is_permit(&self) -> bool {
        self.kind == DecisionKind::Permit
    }

    pub fn is_deny(&self) -> bool {
        self.kind == DecisionKind::Deny
    }

    pub fn trace_strings(&self) -> Vec<String> {
        self.trace.iter().map(|e| e.to_string()).collect()
    }
}

#[derive(Default, Clone)]
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
            // English comment: deny-by-default when no rules exist.
            return Decision {
                kind: DecisionKind::Deny,
                selected_rule: None,
                trace: vec![],
            };
        }

        match ruleset.mode {
            RuleSetMode::AllMustMatch => {
                let mut trace = Vec::new();

                for r in &ruleset.rules {
                    if r.matches(u, a, o, c) {
                        trace.push(TraceEvent {
                            rule: r.name.clone(),
                            result: TraceResult::Match,
                        });
                        continue;
                    }

                    trace.push(TraceEvent {
                        rule: r.name.clone(),
                        result: TraceResult::NoMatch,
                    });

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
                let mut trace = Vec::new();

                for r in &ruleset.rules {
                    if r.matches(u, a, o, c) {
                        trace.push(TraceEvent {
                            rule: r.name.clone(),
                            result: TraceResult::Match,
                        });

                        // English comment: short-circuit permit on first match.
                        return Decision {
                            kind: DecisionKind::Permit,
                            selected_rule: Some(r.name.clone()),
                            trace,
                        };
                    }

                    trace.push(TraceEvent {
                        rule: r.name.clone(),
                        result: TraceResult::NoMatch,
                    });
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
