use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: String, completed: Option<bool>) -> Todo
    where
        Todo: Default,
    {
        Todo {
            name: name,
            completed: completed.unwrap_or(false),
        }
    }
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            completed: false,
        }
    }
}
