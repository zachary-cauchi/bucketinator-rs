use anyhow::{bail, Context, Result};
use log::debug;

use super::todo::{Id, Todo};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

trait TodoDatabase {
    fn load_database(file: PathBuf) -> Result<Vec<Todo>>;
    fn save_database(file: PathBuf, todos: &HashMap<Id, Todo>) -> Result<()>;
}

pub struct TodoLoader;

impl TodoLoader {
    /// Load the todos from the given file path.
    /// Can throw an error when opening a file or during parsing.
    pub fn load_todos(file: PathBuf) -> Result<HashMap<Id, Todo>> {
        let extension = file.extension().and_then(OsStr::to_str);

        // Load the todos from the database according to the file extension.
        let todos: Vec<Todo> = match extension {
            Some("json") => JsonTodoDatabase::load_database(file)
                .with_context(|| format!("db::Failed to parse db file."))?,
            _ => bail!(
                "db::Unsupported db file type ({})",
                file.extension().and_then(OsStr::to_str).unwrap()
            ),
        };

        debug!("Loaded database. Processing contents.");

        Ok(todos
            .into_iter()
            .filter(|t| t.id.is_some())
            .map(|t| (t.id.unwrap(), t))
            .collect())
    }
}

pub struct TodoSaver;

impl TodoSaver {
    /// Save the todos at the given file path.
    /// Can throw an error when opening a file or during serialisation.
    pub fn save_todos(file: PathBuf, todos: &HashMap<Id, Todo>) -> Result<()> {
        let extension = file.extension().and_then(OsStr::to_str);

        // Load the todos from the database according to the file extension.
        match extension {
            Some("json") => JsonTodoDatabase::save_database(file, todos)
                .context("Failed to save todos to db file"),
            Some(ext) => bail!("db::Unsupported db file type ({})", ext),
            None => bail!("db::No file extension found."),
        }
    }
}

struct JsonTodoDatabase;
impl TodoDatabase for JsonTodoDatabase {
    fn load_database(file: PathBuf) -> Result<Vec<Todo>> {
        let db_file: File = File::open(&file)
            .with_context(|| format!("Failed to open file in read-mode ({}).", &file.display()))?;
        let db_file_buffer: BufReader<File> = BufReader::new(db_file);

        debug!("Opened db file for reading. Loading database.");

        serde_json::from_reader(db_file_buffer).context("Deserialisation of todos db failed")
    }

    fn save_database(file: PathBuf, todos: &HashMap<Id, Todo>) -> Result<()> {
        let db_file: File = File::create(&file)
            .with_context(|| format!("Failed to open file in write-mode ({})", file.display()))?;

        let db_file_buffer: BufWriter<File> = BufWriter::new(db_file);

        let values: Vec<&Todo> = todos.values().collect();

        debug!("Opened db file for writing. Writing database.");

        serde_json::to_writer_pretty(db_file_buffer, &values)
            .context("Todos should serialise, but failed for some reason.")
    }
}
