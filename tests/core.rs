// tests/core.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tetthys_cake::{
    d_and, d_not, d_or, pred_d, pred_s, rule, rules_all, rules_any, s_and, s_not, s_or, Action,
    Actor, Context, Engine, ObjectRef,
};

#[test]
fn s_or_identity_false_and_short_circuit() {
    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("thing.do");
    let o = ObjectRef::none();
    let c = Context::default();

    let s0 = s_or!();
    assert_eq!(s0.call(&u, &a, &o, &c), false);

    let hits: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![]));
    let h1 = Arc::clone(&hits);
    let h2 = Arc::clone(&hits);
    let h3 = Arc::clone(&hits);

    let s_false = pred_s!(|_u, _a, _o, _c| {
        h1.lock().unwrap().push("S_false");
        false
    });
    let s_true = pred_s!(|_u, _a, _o, _c| {
        h2.lock().unwrap().push("S_true");
        true
    });
    let s_never = pred_s!(|_u, _a, _o, _c| {
        h3.lock().unwrap().push("S_never");
        true
    });

    let sor = s_or!(s_false, s_true, s_never);
    assert_eq!(sor.call(&u, &a, &o, &c), true);
    assert_eq!(*hits.lock().unwrap(), vec!["S_false", "S_true"]);
}

#[test]
fn s_and_identity_true_and_short_circuit() {
    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("thing.do");
    let o = ObjectRef::none();
    let c = Context::default();

    let s0 = s_and!();
    assert_eq!(s0.call(&u, &a, &o, &c), true);

    let hits: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![]));
    let h1 = Arc::clone(&hits);
    let h2 = Arc::clone(&hits);
    let h3 = Arc::clone(&hits);

    let s_true = pred_s!(|_u, _a, _o, _c| {
        h1.lock().unwrap().push("S_true");
        true
    });
    let s_false = pred_s!(|_u, _a, _o, _c| {
        h2.lock().unwrap().push("S_false");
        false
    });
    let s_never = pred_s!(|_u, _a, _o, _c| {
        h3.lock().unwrap().push("S_never");
        true
    });

    let sand = s_and!(s_true, s_false, s_never);
    assert_eq!(sand.call(&u, &a, &o, &c), false);
    assert_eq!(*hits.lock().unwrap(), vec!["S_true", "S_false"]);
}

#[test]
fn s_not_negates() {
    let u = Actor::new("u-1", vec!["admin".to_string()]);
    let a = Action::new("thing.do");
    let o = ObjectRef::none();
    let c = Context::default();

    let s_true = pred_s!(|_u| true);
    let s_false = pred_s!(|_u| false);

    assert_eq!(s_not!(s_true).call(&u, &a, &o, &c), false);
    assert_eq!(s_not!(s_false).call(&u, &a, &o, &c), true);
}

#[test]
fn d_or_identity_false_and_short_circuit() {
    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("thing.do");
    let o = ObjectRef::none();
    let c = Context::default();

    let d0 = d_or!();
    assert_eq!(d0.call(&u, &a, &o, &c), false);

    let hits: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![]));
    let h1 = Arc::clone(&hits);
    let h2 = Arc::clone(&hits);
    let h3 = Arc::clone(&hits);

    let d_false = pred_d!(|_u, _a, _o| {
        h1.lock().unwrap().push("D_false");
        false
    });
    let d_true = pred_d!(|_u, _a, _o| {
        h2.lock().unwrap().push("D_true");
        true
    });
    let d_never = pred_d!(|_u, _a, _o| {
        h3.lock().unwrap().push("D_never");
        true
    });

    let dor = d_or!(d_false, d_true, d_never);
    assert_eq!(dor.call(&u, &a, &o, &c), true);
    assert_eq!(*hits.lock().unwrap(), vec!["D_false", "D_true"]);
}

#[test]
fn d_and_identity_true_and_short_circuit() {
    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("thing.do");
    let o = ObjectRef::none();
    let c = Context::default();

    let d0 = d_and!();
    assert_eq!(d0.call(&u, &a, &o, &c), true);

    let hits: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![]));
    let h1 = Arc::clone(&hits);
    let h2 = Arc::clone(&hits);
    let h3 = Arc::clone(&hits);

    let d_true = pred_d!(|_u, _a, _o| {
        h1.lock().unwrap().push("D_true");
        true
    });
    let d_false = pred_d!(|_u, _a, _o| {
        h2.lock().unwrap().push("D_false");
        false
    });
    let d_never = pred_d!(|_u, _a, _o| {
        h3.lock().unwrap().push("D_never");
        true
    });

    let dand = d_and!(d_true, d_false, d_never);
    assert_eq!(dand.call(&u, &a, &o, &c), false);
    assert_eq!(*hits.lock().unwrap(), vec!["D_true", "D_false"]);
}

#[test]
fn d_not_negates() {
    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("thing.do");
    let o = ObjectRef::none();
    let c = Context::default();

    let d_true = pred_d!(|_u, _a, _o| true);
    let d_false = pred_d!(|_u, _a, _o| false);

    assert_eq!(d_not!(d_true).call(&u, &a, &o, &c), false);
    assert_eq!(d_not!(d_false).call(&u, &a, &o, &c), true);
}

#[test]
fn combinators_evaluate_left_to_right_when_no_short_circuit() {
    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("x");
    let o = ObjectRef::none();
    let c = Context::default();

    let hits: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![]));
    let h1 = Arc::clone(&hits);
    let h2 = Arc::clone(&hits);
    let h3 = Arc::clone(&hits);

    // OR: all false => must evaluate all, in order
    let p1 = pred_s!(|_u, _a, _o, _c| {
        h1.lock().unwrap().push("1");
        false
    });
    let p2 = pred_s!(|_u, _a, _o, _c| {
        h2.lock().unwrap().push("2");
        false
    });
    let p3 = pred_s!(|_u, _a, _o, _c| {
        h3.lock().unwrap().push("3");
        false
    });

    let por = s_or!(p1, p2, p3);
    assert_eq!(por.call(&u, &a, &o, &c), false);
    assert_eq!(*hits.lock().unwrap(), vec!["1", "2", "3"]);
}

#[test]
fn rules_all_denies_as_soon_as_a_rule_fails() {
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
fn rules_all_permits_only_when_every_rule_matches() {
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
fn rules_all_denies_when_no_rules_exist_deny_by_default() {
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
fn rules_any_permits_as_soon_as_a_rule_matches_short_circuit_and_selects_first_match() {
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
fn rules_any_denies_when_none_match() {
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
fn rules_any_denies_when_no_rules_exist_deny_by_default() {
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

#[test]
fn pred_s_supports_arity_1_and_arity_4() {
    let u = Actor::new("u-1", vec!["admin".to_string()]);
    let a = Action::new("post.update");
    let o = ObjectRef::none();
    let c = Context::new(HashMap::from([("ip".to_string(), "127.0.0.1".to_string())]));

    let s_admin_only = pred_s!(|u| u.has_role("admin"));
    assert_eq!(s_admin_only.call(&u, &a, &o, &c), true);

    let s_ip_is_local = pred_s!(|_u, _a, _o, c| c.get("ip") == Some("127.0.0.1"));
    assert_eq!(s_ip_is_local.call(&u, &a, &o, &c), true);
}

#[derive(Debug)]
struct Post {
    owner_id: String,
}
impl Post {
    fn owner_id(&self) -> &str {
        &self.owner_id
    }
}

#[test]
fn objectref_arc_downcast_works_and_mismatch_returns_none() {
    let post = Arc::new(Post {
        owner_id: "u-1".to_string(),
    });

    let o_post = ObjectRef::new_arc("Post", Arc::clone(&post));
    assert!(o_post.arc::<Post>().is_some());

    #[derive(Debug)]
    struct Order {
        _id: String,
    }

    // Type mismatch: stored Post, requested Order
    assert!(o_post.arc::<Order>().is_none());

    // None object
    let o_none = ObjectRef::none();
    assert!(o_none.arc::<Post>().is_none());
}

#[test]
fn pred_d_can_read_objectref_and_actor_for_domain_checks() {
    let post = Arc::new(Post {
        owner_id: "u-1".to_string(),
    });

    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("post.update");
    let o = ObjectRef::new_arc("Post", post);
    let c = Context::default();

    let d_owner = pred_d!(|u, _a, o| {
        o.arc::<Post>()
            .is_some_and(|p| p.owner_id() == u.id.as_str())
    });

    assert_eq!(d_owner.call(&u, &a, &o, &c), true);

    let u2 = Actor::new("u-2", vec!["user".to_string()]);
    assert_eq!(d_owner.call(&u2, &a, &o, &c), false);
}
