use std::sync::{Arc, Mutex};

use tetthys_cake::{d_and, d_not, d_or, pred_d, pred_s, s_and, s_not, s_or, Action, Actor, Context, ObjectRef};

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
