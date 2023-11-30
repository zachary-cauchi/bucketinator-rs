use serde::{Deserialize, Serialize};

pub type Id = usize;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Option<Id>,
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: String, completed: Option<bool>) -> Todo
    where
        Todo: Default,
    {
        Todo {
            id: None,
            name: name,
            completed: completed.unwrap_or(false),
        }
    }
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            completed: false,
        }
    }
}
