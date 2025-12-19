pub mod actor;
pub mod policy;
pub mod registry;
pub mod layer;
pub mod error;

pub use actor::{ActorResolver, DefaultActorResolver};
pub use policy::{PolicyDecl, ObjectRequirement, norm_method, norm_resource};
pub use registry::{Cake, CakeState, CakeConfig};
pub use layer::CakeLayer;
pub use error::CakeError;

/// English comment: Policy auto-registration macro.
/// Usage:
/// cake_policy!(resource="post", method="update", priority=0, object=Required, fn post_update(u,a,o,c) -> RuleSet { ... });
#[macro_export]
macro_rules! cake_policy {
    (
        resource=$res:literal,
        method=$method:literal,
        priority=$prio:expr,
        object=$objreq:ident,
        fn $name:ident($u:ident,$a:ident,$o:ident,$c:ident) -> $ret:ty $body:block
    ) => {
        fn $name(
            $u: &$crate::Actor,
            $a: &$crate::Action,
            $o: &$crate::ObjectRef,
            $c: &$crate::Context,
        ) -> $ret $body

        inventory::submit! {
            $crate::integration::cake::policy::PolicyDecl {
                resource: $res,
                method: $method,
                priority: $prio,
                object_requirement: $crate::integration::cake::policy::ObjectRequirement::$objreq,
                f: $name,
            }
        }
    };
}

/// English comment: Syntactic sugar default priority=0 object=Optional.
#[macro_export]
macro_rules! cake_policy_simple {
    (
        resource=$res:literal,
        method=$method:literal,
        fn $name:ident($u:ident,$a:ident,$o:ident,$c:ident) -> $ret:ty $body:block
    ) => {
        $crate::cake_policy!(
            resource=$res,
            method=$method,
            priority=0,
            object=Optional,
            fn $name($u,$a,$o,$c) -> $ret $body
        );
    };
}

/// English comment: User-facing macros with unified names.
/// - can!(cake, "post.update") OR can!(cake, "post.update", obj)
/// - authorize!(cake, "post.update") OR authorize!(cake, "post.update", obj)
#[macro_export]
macro_rules! can {
    ($cake:expr, $action:expr) => {{
        $cake.can($action, ())
    }};
    ($cake:expr, $action:expr, $obj:expr) => {{
        $cake.can($action, $obj)
    }};
}

#[macro_export]
macro_rules! authorize {
    ($cake:expr, $action:expr) => {{
        $cake.authorize($action, ())
    }};
    ($cake:expr, $action:expr, $obj:expr) => {{
        $cake.authorize($action, $obj)
    }};
}
