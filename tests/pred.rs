use std::collections::HashMap;

use tetthys_cake::{pred_d, pred_s, Action, Actor, Context, ObjectRef};

#[test]
fn builds_subject_predicate_from_a_closure() {
    let u = Actor::new("u-1", vec!["admin".to_string()]);
    let a = Action::new("post.update");
    let o = ObjectRef::new("Post", ());
    let c = Context::new(HashMap::from([(
        "ip".to_string(),
        "127.0.0.1".to_string(),
    )]));

    let s_admin_only = pred_s!(|u| u.has_role("admin"));
    assert_eq!(s_admin_only.call(&u, &a, &o, &c), true);

    let u2 = Actor::new("u-2", vec!["user".to_string()]);
    assert_eq!(s_admin_only.call(&u2, &a, &o, &c), false);
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
fn builds_domain_predicate_from_a_closure() {
    let post = Post {
        owner_id: "u-1".to_string(),
    };

    let u = Actor::new("u-1", vec!["user".to_string()]);
    let a = Action::new("post.update");
    let o = ObjectRef::new("Post", post);
    let c = Context::default();

    let d_owner = pred_d!(|u, _a, o| {
        o.downcast_ref::<Post>()
            .is_some_and(|p| p.owner_id() == u.id.as_str())
    });

    assert_eq!(d_owner.call(&u, &a, &o, &c), true);

    let u2 = Actor::new("u-2", vec!["user".to_string()]);
    assert_eq!(d_owner.call(&u2, &a, &o, &c), false);
}

#[derive(Debug)]
struct Order {
    approver_id: String,
    status: String,
}
impl Order {
    fn approver_id(&self) -> &str {
        &self.approver_id
    }
    fn status(&self) -> &str {
        &self.status
    }
}

#[test]
fn supports_combining_s_and_d_via_closures_for_readability() {
    let u = Actor::new("u-9", vec!["manager".to_string()]);
    let a = Action::new("order.approve");

    let o = ObjectRef::new(
        "Order",
        Order {
            approver_id: "u-9".to_string(),
            status: "paid".to_string(),
        },
    );

    let c = Context::new(HashMap::from([("tenant".to_string(), "acme".to_string())]));

    let s_manager = pred_s!(|u| u.has_role("manager"));

    let d_paid = pred_d!(|_u, _a, o| {
        o.downcast_ref::<Order>()
            .is_some_and(|ord| ord.status() == "paid")
    });

    let d_can_approve = pred_d!(|u, _a, o| {
        o.downcast_ref::<Order>()
            .is_some_and(|ord| ord.approver_id() == u.id.as_str())
    });

    assert_eq!(s_manager.call(&u, &a, &o, &c), true);
    assert_eq!(d_paid.call(&u, &a, &o, &c), true);
    assert_eq!(d_can_approve.call(&u, &a, &o, &c), true);
}
