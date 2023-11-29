use std::path::{Path, PathBuf};

use crate::{
    config::BucketinatorConfiguration,
    model::{db::TodoLoader, todo::Todo},
};

pub struct App {
    pub is_initialized: bool,
    state_changed: bool,
    pub todos: Option<Vec<Todo>>,
    pub conf: BucketinatorConfiguration,
}

impl App {
    pub fn new(conf: BucketinatorConfiguration) -> App {
        App {
            is_initialized: false,
            state_changed: false,
            todos: None,
            conf,
        }
    }

    pub fn initialize(&mut self) {
        if self.is_initialized {
            return;
        }

        self.load_todos();
    }

    fn load_todos(&mut self) {
        println!("Loading todos from file {}", self.conf.db_file_path);

        let file = match Self::validate_file(self.conf.db_file_path.as_str()) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        self.todos = Some(TodoLoader::load_todos(file));
        self.is_initialized = true;

        println!("Loaded {} todos.", self.todos.as_ref().unwrap().len());
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        self.todos.as_ref().unwrap()
    }

    fn get_mut_todos(&mut self) -> &mut Vec<Todo> {
        self.todos.as_mut().unwrap()
    }

    pub fn add_todo(&mut self, todo: Todo) -> Option<&Todo> {
        self.get_mut_todos().push(todo);
        self.state_changed = true;

        self.get_todos().last()
    }

    fn validate_file(raw_path: &str) -> Result<PathBuf, String> {
        let path = Path::new(raw_path);

        if path.is_file() {
            Ok(PathBuf::from(raw_path))
        } else {
            Err(format!(
                "Failed to validate file path '{}': Path is a not a file or doesn't exist.",
                raw_path
            ))
        }
    }
}
