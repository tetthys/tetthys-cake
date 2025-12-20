// model.rs
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Actor {
    pub id: String,
    pub roles: Vec<String>,
}

impl Actor {
    pub fn new(id: impl Into<String>, roles: impl Into<Vec<String>>) -> Self {
        Self { id: id.into(), roles: roles.into() }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
}

impl Action {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Debug, Clone)]
pub enum ObjectRef {
    None,
    Some {
        kind: String,
        data: Arc<dyn Any + Send + Sync>,
    },
}

impl ObjectRef {
    pub fn none() -> Self {
        Self::None
    }

    pub fn is_none(&self) -> bool {
        matches!(self, ObjectRef::None)
    }

    pub fn kind(&self) -> Option<&str> {
        match self {
            ObjectRef::None => None,
            ObjectRef::Some { kind, .. } => Some(kind.as_str()),
        }
    }

    pub fn new_arc<T: Any + Send + Sync>(kind: impl Into<String>, data: Arc<T>) -> Self {
        Self::Some { kind: kind.into(), data }
    }

    pub fn arc<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        match self {
            ObjectRef::None => None,
            ObjectRef::Some { data, .. } => {
                let cloned: Arc<dyn Any + Send + Sync> = Arc::clone(data);
                Arc::downcast::<T>(cloned).ok()
            }
        }
    }
}

pub trait IntoObjectRef {
    fn into_object_ref(self) -> ObjectRef;
}

impl IntoObjectRef for ObjectRef {
    fn into_object_ref(self) -> ObjectRef {
        self
    }
}

/// English comment: Unit means "no object".
impl IntoObjectRef for () {
    fn into_object_ref(self) -> ObjectRef {
        ObjectRef::none()
    }
}

impl<T: Any + Send + Sync> IntoObjectRef for Arc<T> {
    fn into_object_ref(self) -> ObjectRef {
        ObjectRef::new_arc(std::any::type_name::<T>(), self)
    }
}

impl<T: Any + Send + Sync> IntoObjectRef for Option<Arc<T>> {
    fn into_object_ref(self) -> ObjectRef {
        match self {
            None => ObjectRef::none(),
            Some(v) => ObjectRef::new_arc(std::any::type_name::<T>(), v),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub data: HashMap<String, String>,
}

impl Context {
    pub fn new(data: HashMap<String, String>) -> Self {
        Self { data }
    }

    pub fn get(&self, k: &str) -> Option<&str> {
        self.data.get(k).map(|s| s.as_str())
    }
}
