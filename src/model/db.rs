use super::todo::Todo;
use std::{error::Error, ffi::OsStr, fs::File, io::BufReader, path::PathBuf};

trait TodoDatabase {
    fn load_database(file: PathBuf) -> Result<Vec<Todo>, Box<dyn Error>>;
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

struct JsonTodoDatabase;
impl TodoDatabase for JsonTodoDatabase {
    fn load_database(file: PathBuf) -> Result<Vec<Todo>, Box<dyn Error>> {
        let db_file = File::open(file)?;
        let db_file_buffer = BufReader::new(db_file);

        let todos = serde_json::from_reader(db_file_buffer)?;

        Ok(todos)
    }
}
