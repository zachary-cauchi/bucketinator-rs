use anyhow::{bail, Context, Result};
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};

use crate::model::todo::{Id, Todo};

use super::{json_database::JsonTodoDatabase, sqlite_database::SQLiteTodoDatabase};

pub trait TodoDatabase {
    fn load_database(self: &Self) -> Result<Vec<Todo>>;
    fn save_database(self: &Self, todos: &HashMap<Id, Todo>) -> Result<()>;
}

pub struct TodoDatabaseBuilder;

impl TodoDatabaseBuilder {
    pub fn get_todo_database(db_location: PathBuf) -> Result<Box<dyn TodoDatabase>> {
        let db_existence_check: Result<bool> = db_location
            .try_exists()
            .context("Could not check if db location exists.");

        if !db_existence_check? {
            bail!(format!(
                "Failed to validate file path '{}': Path is a not a file or doesn't exist.",
                db_location.display()
            ));
        }

        let extension_option = db_location.extension().and_then(OsStr::to_str);

        // Load the todos from the database according to the file extension.
        match extension_option {
            Some("json") => Ok(Box::new(JsonTodoDatabase::new(db_location))),
            Some("db3") => Ok(Box::new(SQLiteTodoDatabase::new(db_location))),
            Some(ext) => bail!("db::Unsupported db file type ({})", ext),
            None => bail!("db::No file extension found."),
        }
    }
}
