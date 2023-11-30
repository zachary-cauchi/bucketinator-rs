use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    config::BucketinatorConfiguration,
    model::{
        db::{TodoLoader, TodoSaver},
        todo::{Id, Todo},
    },
};

pub struct App {
    pub is_initialized: bool,
    state_changed: bool,
    last_id: Id,
    pub todos: Option<HashMap<Id, Todo>>,
    pub conf: BucketinatorConfiguration,
}

impl App {
    pub fn new(conf: BucketinatorConfiguration) -> App {
        App {
            is_initialized: false,
            state_changed: false,
            last_id: 1,
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

    pub fn save_state(&mut self) {
        self.save_todos();

        self.state_changed = false
    }

    fn load_todos(&mut self) {
        println!("Loading todos from file {}", self.conf.db_file_path);

        let file = match Self::validate_file(self.conf.db_file_path.as_str()) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        self.todos = Some(TodoLoader::load_todos(file));
        self.last_id = self
            .get_todos()
            .keys()
            .fold(self.last_id, |acc, i| acc.max(*i));
        self.is_initialized = true;

        println!("Loaded {} todos.", self.todos.as_ref().unwrap().len());
    }

    fn save_todos(&mut self) {
        println!("Saving todos to file {}", self.conf.db_file_path);

        let file = match Self::validate_file(self.conf.db_file_path.as_str()) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        TodoSaver::save_todos(file, self.get_todos());
    }

    pub fn get_todos(&self) -> &HashMap<Id, Todo> {
        self.todos.as_ref().unwrap()
    }

    fn get_mut_todos(&mut self) -> &mut HashMap<Id, Todo> {
        self.todos.as_mut().unwrap()
    }

    pub fn add_todo(&mut self, mut todo: Todo) -> Option<&Todo> {
        self.last_id += 1;
        let _ = todo.id.insert(self.last_id);

        let new_id = self.last_id;
        self.get_mut_todos().insert(new_id, todo);

        self.state_changed = true;

        self.save_state();

        self.get_todos().get(&self.last_id)
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
