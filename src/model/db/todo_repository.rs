use anyhow::{Context, Result};
use log::debug;

use std::{collections::HashMap, path::PathBuf};

use crate::model::{
    db::todo_database::TodoDatabase,
    todo::{Id, Todo},
};

use super::todo_database::TodoDatabaseBuilder;

pub struct TodoRepository {
    database: Box<dyn TodoDatabase>,
}

impl TodoRepository {
    pub fn new(db_location: PathBuf) -> Self {
        let todo_database = match TodoDatabaseBuilder::get_todo_database(db_location) {
            Ok(db) => db,
            Err(e) => panic!("Failed to initialise todo database: {:?}", e),
        };

        TodoRepository {
            database: todo_database,
        }
    }

    pub fn load_todos(self: &Self) -> Result<HashMap<Id, Todo>> {
        let todos: Vec<Todo> = self
            .database
            .load_database()
            .context("Failed to load todos from database.")?;

        debug!("Loaded database. Processing contents.");

        Ok(todos.into_iter().map(|t| (t.id, t)).collect())
    }

    pub fn save_todos(self: &Self, todos: &HashMap<Id, Todo>) -> Result<()> {
        self.database
            .save_database(todos)
            .context("Failed to save todos to database.")
    }
}
