use std::any::Any;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Actor {
    pub id: String,
    pub roles: Vec<String>,
}

impl Actor {
    pub fn new(id: impl Into<String>, roles: impl Into<Vec<String>>) -> Self {
        Self {
            id: id.into(),
            roles: roles.into(),
        }
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

#[derive(Debug)]
pub struct ObjectRef {
    pub kind: String,
    pub data: Box<dyn Any + Send + Sync>,
}

impl ObjectRef {
    pub fn new<T: Any + Send + Sync>(kind: impl Into<String>, data: T) -> Self {
        Self {
            kind: kind.into(),
            data: Box::new(data),
        }
    }

    /// English comment: Downcast helper for domain predicates.
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
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
