use std::any::Any;
use std::sync::Arc;

use leptos::prelude::*;

use crate::integration::cake::registry::Cake;

/// ---------------------------------------------------------------------
/// Leptos UI Context Wrapper
/// ---------------------------------------------------------------------
///
/// English comment:
/// - Wraps Cake for Leptos UI usage
/// - Stored in Leptos context
/// - Fail-closed when missing (important for SSR/CSR safety)
///
#[derive(Clone)]
pub struct CakeUi {
    cake: Option<Cake>,
}

impl CakeUi {
    /// English comment: Create UI wrapper from Cake (SSR entry).
    pub fn new(cake: Cake) -> Self {
        Self { cake: Some(cake) }
    }

    /// English comment: Empty UI wrapper (used when context is missing).
    pub fn empty() -> Self {
        Self { cake: None }
    }

    /// English comment: Check permission without an object.
    /// - Returns false if Cake is missing.
    pub fn can(&self, action: &'static str) -> bool {
        self.cake
            .as_ref()
            .is_some_and(|cake| cake.can(action, ()))
    }

    /// English comment: Check permission with an object (Arc<T> recommended).
    /// - Returns false if Cake is missing.
    pub fn can_obj<T>(&self, action: &'static str, obj: Arc<T>) -> bool
    where
        T: Any + Send + Sync + 'static,
    {
        self.cake
            .as_ref()
            .is_some_and(|cake| cake.can(action, obj))
    }

    /// English comment: Expose underlying Cake if needed (advanced/debug).
    pub fn cake(&self) -> Option<&Cake> {
        self.cake.as_ref()
    }
}

/// ---------------------------------------------------------------------
/// Context helpers
/// ---------------------------------------------------------------------

/// English comment:
/// - Call this ONCE at SSR entry
/// - Makes Cake available to all Leptos components
pub fn provide_cake(cake: Cake) {
    provide_context(CakeUi::new(cake));
}

/// English comment:
/// - Get CakeUi from Leptos context
/// - If missing, return empty (fail-closed)
pub fn use_cake_ui() -> CakeUi {
    use_context::<CakeUi>().unwrap_or_else(CakeUi::empty)
}

/// ---------------------------------------------------------------------
/// Low-level helper functions (used by macros)
/// ---------------------------------------------------------------------

/// English comment: Blade-like @can (no object)
pub fn cake_can(action: &'static str) -> bool {
    use_cake_ui().can(action)
}

/// English comment: Blade-like @can (with object)
pub fn cake_can_obj<T>(action: &'static str, obj: Arc<T>) -> bool
where
    T: Any + Send + Sync + 'static,
{
    use_cake_ui().can_obj(action, obj)
}

/// ---------------------------------------------------------------------
/// Blade-inspired macros (UI only, fail-closed)
/// ---------------------------------------------------------------------

#[macro_export]
macro_rules! cake_can {
    ($action:expr) => {{
        $crate::integration::cake::leptos::cake_can($action)
    }};
    ($action:expr, $obj:expr) => {{
        $crate::integration::cake::leptos::cake_can_obj($action, $obj)
    }};
}

#[macro_export]
macro_rules! cake_cannot {
    ($action:expr) => {{
        !$crate::cake_can!($action)
    }};
    ($action:expr, $obj:expr) => {{
        !$crate::cake_can!($action, $obj)
    }};
}

#[macro_export]
macro_rules! cake_show {
    ($action:expr, $body:expr) => {{
        if $crate::cake_can!($action) {
            $body
        } else {
            leptos::prelude::view! { <></> }
        }
    }};
    ($action:expr, $obj:expr, $body:expr) => {{
        if $crate::cake_can!($action, $obj) {
            $body
        } else {
            leptos::prelude::view! { <></> }
        }
    }};
}

#[macro_export]
macro_rules! cake_hide {
    ($action:expr, $body:expr) => {{
        if !$crate::cake_can!($action) {
            $body
        } else {
            leptos::prelude::view! { <></> }
        }
    }};
    ($action:expr, $obj:expr, $body:expr) => {{
        if !$crate::cake_can!($action, $obj) {
            $body
        } else {
            leptos::prelude::view! { <></> }
        }
    }};
}

#[macro_export]
macro_rules! cake_if {
    ($action:expr, $then:expr, $else:expr) => {{
        if $crate::cake_can!($action) { $then } else { $else }
    }};
    ($action:expr, $obj:expr, $then:expr, $else:expr) => {{
        if $crate::cake_can!($action, $obj) { $then } else { $else }
    }};
}

#[macro_export]
macro_rules! cake_unless {
    ($action:expr, $body:expr) => {{
        if !$crate::cake_can!($action) {
            $body
        } else {
            leptos::prelude::view! { <></> }
        }
    }};
    ($action:expr, $obj:expr, $body:expr) => {{
        if !$crate::cake_can!($action, $obj) {
            $body
        } else {
            leptos::prelude::view! { <></> }
        }
    }};
}

#[macro_export]
macro_rules! cake_disabled {
    ($action:expr) => {{
        if $crate::cake_can!($action) { "" } else { "disabled" }
    }};
    ($action:expr, $obj:expr) => {{
        if $crate::cake_can!($action, $obj) { "" } else { "disabled" }
    }};
}

#[macro_export]
macro_rules! cake_class {
    ($action:expr, $class:expr) => {{
        if $crate::cake_can!($action) { $class } else { "" }
    }};
    ($action:expr, $obj:expr, $class:expr) => {{
        if $crate::cake_can!($action, $obj) { $class } else { "" }
    }};
}
