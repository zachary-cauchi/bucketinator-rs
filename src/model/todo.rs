use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

pub type Id = i32;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::model::db::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Id,
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: String, completed: Option<bool>) -> Todo
    where
        Todo: Default,
    {
        Todo {
            id: 0,
            name: name,
            completed: completed.unwrap_or(false),
        }
    }
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            completed: false,
        }
    }
}
