use std::sync::{Arc, Mutex};

use tetthys_cake::{pred_d, pred_s, rule, rules_all, rules_any, Action, Actor, Context, Engine, ObjectRef};

#[test]
fn all_must_match_denies_as_soon_as_a_rule_fails() {
    let calls: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![]));

    let c1 = Arc::clone(&calls);
    let c2 = Arc::clone(&calls);

    let rules = rules_all![
        rule!(
            "First",
            pred_s!(|_u, _a, _o, _c| {
                c1.lock().unwrap().push("first");
                false
            }),
            pred_d!(|_u, _a, _o| true)
        ),
        rule!(
            "Second",
            pred_s!(|_u, _a, _o, _c| {
                c2.lock().unwrap().push("second");
                true
            }),
            pred_d!(|_u, _a, _o| true)
        ),
    ];

    let decision = Engine::new().decide(
        &Actor::new("u", vec!["user".to_string()]),
        &Action::new("post.update"),
        &ObjectRef::none(),
        &Context::default(),
        &rules,
    );

    assert!(decision.is_deny());
    assert_eq!(decision.trace_strings(), vec!["[First] no-match"]);
    assert_eq!(*calls.lock().unwrap(), vec!["first"]);
}

#[test]
fn all_must_match_permits_only_when_every_rule_matches() {
    let rules = rules_all![
        rule!("Owner", pred_s!(|_u| true), pred_d!(|_u, _a, _o| true)),
        rule!("Recent", pred_s!(|_u| true), pred_d!(|_u, _a, _o| true)),
    ];

    let decision = Engine::new().decide(
        &Actor::new("u", vec![]),
        &Action::new("post.update"),
        &ObjectRef::none(),
        &Context::default(),
        &rules,
    );

    assert!(decision.is_permit());
    assert_eq!(decision.selected_rule.as_deref(), Some("all-rules"));
    assert_eq!(decision.trace_strings(), vec!["[Owner] match", "[Recent] match"]);
}

#[test]
fn all_must_match_denies_when_no_rules_exist_deny_by_default() {
    let rules = rules_all![];

    let decision = Engine::new().decide(
        &Actor::new("u", vec![]),
        &Action::new("post.update"),
        &ObjectRef::none(),
        &Context::default(),
        &rules,
    );

    assert!(decision.is_deny());
    assert!(decision.trace.is_empty());
    assert!(decision.selected_rule.is_none());
}

#[test]
fn any_may_match_permits_as_soon_as_a_rule_matches_short_circuit() {
    let calls: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![]));

    let c1 = Arc::clone(&calls);
    let c2 = Arc::clone(&calls);
    let c3 = Arc::clone(&calls);

    let rules = rules_any![
        rule!(
            "First",
            pred_s!(|_u, _a, _o, _c| {
                c1.lock().unwrap().push("first");
                false
            }),
            pred_d!(|_u, _a, _o| true)
        ),
        rule!(
            "Second",
            pred_s!(|_u, _a, _o, _c| {
                c2.lock().unwrap().push("second");
                true
            }),
            pred_d!(|_u, _a, _o| true)
        ),
        rule!(
            "Third",
            pred_s!(|_u, _a, _o, _c| {
                c3.lock().unwrap().push("third");
                true
            }),
            pred_d!(|_u, _a, _o| true)
        ),
    ];

    let decision = Engine::new().decide(
        &Actor::new("u", vec!["user".to_string()]),
        &Action::new("post.update"),
        &ObjectRef::none(),
        &Context::default(),
        &rules,
    );

    assert!(decision.is_permit());
    assert_eq!(decision.selected_rule.as_deref(), Some("Second"));
    assert_eq!(decision.trace_strings(), vec!["[First] no-match", "[Second] match"]);
    assert_eq!(*calls.lock().unwrap(), vec!["first", "second"]);
}

#[test]
fn any_may_match_denies_when_none_match() {
    let rules = rules_any![
        rule!("A", pred_s!(|_u| false), pred_d!(|_u, _a, _o| true)),
        rule!("B", pred_s!(|_u| false), pred_d!(|_u, _a, _o| true)),
    ];

    let decision = Engine::new().decide(
        &Actor::new("u", vec![]),
        &Action::new("post.update"),
        &ObjectRef::none(),
        &Context::default(),
        &rules,
    );

    assert!(decision.is_deny());
    assert!(decision.selected_rule.is_none());
    assert_eq!(decision.trace_strings(), vec!["[A] no-match", "[B] no-match"]);
}

#[test]
fn any_may_match_denies_when_no_rules_exist_deny_by_default() {
    let rules = rules_any![];

    let decision = Engine::new().decide(
        &Actor::new("u", vec![]),
        &Action::new("post.update"),
        &ObjectRef::none(),
        &Context::default(),
        &rules,
    );

    assert!(decision.is_deny());
    assert!(decision.trace.is_empty());
    assert!(decision.selected_rule.is_none());
}
