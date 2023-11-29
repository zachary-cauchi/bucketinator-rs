use super::todo::Todo;
use std::{
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

trait TodoDatabase {
    fn load_database(file: PathBuf) -> Result<Vec<Todo>, Box<dyn Error>>;
    fn save_database(file: PathBuf, todos: &Vec<Todo>) -> Result<(), Box<dyn Error>>;
}

pub struct TodoLoader;

impl TodoLoader {
    /// Load the todos from the given file path.
    /// Can throw an error when opening a file or during parsing.
    pub fn load_todos(file: PathBuf) -> Vec<Todo> {
        let extension = file.extension().and_then(OsStr::to_str);

        // Load the todos from the database according to the file extension.
        let db_load_result = match extension {
            Some("json") => JsonTodoDatabase::load_database(file),
            _ => panic!(
                "Unsupported db file type ({})",
                file.extension().and_then(OsStr::to_str).unwrap()
            ),
        };

        db_load_result.unwrap_or_else(|msg| panic!("Failed to load db file: {}", msg))
    }
}

pub struct TodoSaver;

impl TodoSaver {
    /// Load the todos from the given file path.
    /// Can throw an error when opening a file or during parsing.
    pub fn save_todos(file: PathBuf, todos: &Vec<Todo>) {
        let extension = file.extension().and_then(OsStr::to_str);

        // Load the todos from the database according to the file extension.
        let db_load_result = match extension {
            Some("json") => JsonTodoDatabase::save_database(file, todos),
            _ => panic!(
                "Unsupported db file type ({})",
                file.extension().and_then(OsStr::to_str).unwrap()
            ),
        };

        db_load_result.unwrap_or_else(|msg| panic!("Failed to save todos to db file: {}", msg))
    }
}

struct JsonTodoDatabase;
impl TodoDatabase for JsonTodoDatabase {
    fn load_database(file: PathBuf) -> Result<Vec<Todo>, Box<dyn Error>> {
        let db_file = File::open(file)?;
        let db_file_buffer = BufReader::new(db_file);

        let todos = serde_json::from_reader(db_file_buffer)?;

        Ok(todos)
    }

    fn save_database(file: PathBuf, todos: &Vec<Todo>) -> Result<(), Box<dyn Error>> {
        let db_file = File::create(file)?;
        let db_file_buffer = BufWriter::new(db_file);

        Ok(serde_json::to_writer_pretty(db_file_buffer, &todos)
            .expect("Todos should serialise, but failed for some reason."))
    }
}
