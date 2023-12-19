use diesel::prelude::*;
use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use diesel::{Connection, SqliteConnection};

use crate::model::todo::{Id, Todo};

use super::todo_database::TodoDatabase;

pub struct SQLiteTodoDatabase {
    db_location: PathBuf,
}

impl SQLiteTodoDatabase {
    pub fn new(db_location: PathBuf) -> Self {
        SQLiteTodoDatabase {
            db_location: db_location,
        }
    }
}

fn get_database_connection(conn_str: &str) -> SqliteConnection {
    SqliteConnection::establish(conn_str)
        .context("Could not establish connection to SQLite database.")
        .expect("Connection should have been established")
}

impl TodoDatabase for SQLiteTodoDatabase {
    fn load_database(self: &Self) -> Result<Vec<Todo>> {
        use crate::model::db::schema::todos::dsl::*;

        let mut conn: SqliteConnection =
            get_database_connection(self.db_location.to_str().unwrap());

        todos
            .select(Todo::as_select())
            .load(&mut conn)
            .context("Failed to get todos from database.")
    }

    fn save_database(self: &Self, new_todos: &HashMap<Id, Todo>) -> Result<()> {
        use crate::model::db::schema::todos::dsl::*;

        let mut conn: SqliteConnection =
            get_database_connection(self.db_location.to_str().unwrap());

        let todos_to_replace: Vec<Todo> = new_todos.values().into_iter().cloned().collect();

        diesel::replace_into(todos)
            .values(todos_to_replace)
            .execute(&mut conn)
            .context("Failed to update database with new todos.")
            .map(|_| ())
    }
}
