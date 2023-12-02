use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::{Context, Result};
use log::debug;

use crate::model::todo::{Id, Todo};

use super::todo_database::TodoDatabase;

pub struct JsonTodoDatabase {
    db_location: PathBuf,
}

impl JsonTodoDatabase {
    pub fn new(db_location: PathBuf) -> Self {
        JsonTodoDatabase {
            db_location: db_location,
        }
    }
}

impl TodoDatabase for JsonTodoDatabase {
    fn load_database(self: &Self) -> Result<Vec<Todo>> {
        let db_file: File = File::open(&self.db_location).with_context(|| {
            format!(
                "Failed to open file in read-mode ({}).",
                &self.db_location.display()
            )
        })?;
        let db_file_buffer: BufReader<File> = BufReader::new(db_file);

        debug!("Opened db file for reading. Loading database.");

        serde_json::from_reader(db_file_buffer).context("Deserialisation of todos db failed")
    }

    fn save_database(self: &Self, todos: &HashMap<Id, Todo>) -> Result<()> {
        let db_file: File = File::create(&self.db_location).with_context(|| {
            format!(
                "Failed to open file in write-mode ({})",
                self.db_location.display()
            )
        })?;

        let db_file_buffer: BufWriter<File> = BufWriter::new(db_file);

        let values: Vec<&Todo> = todos.values().collect();

        debug!("Opened db file for writing. Writing database.");

        serde_json::to_writer_pretty(db_file_buffer, &values)
            .context("Todos should serialise, but failed for some reason.")
    }
}
