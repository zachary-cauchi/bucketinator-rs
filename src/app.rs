use std::{collections::HashMap, path::PathBuf};

use crate::{
    config::BucketinatorConfiguration,
    model::{
        db::todo_repository::TodoRepository,
        todo::{Id, Todo},
    },
};

use log::info;

pub struct App {
    pub is_initialized: bool,
    state_changed: bool,
    last_id: Id,
    pub todos: Option<HashMap<Id, Todo>>,
    pub conf: BucketinatorConfiguration,
    todo_repository: Option<TodoRepository>,
}

impl App {
    pub fn new(conf: BucketinatorConfiguration) -> App {
        App {
            is_initialized: false,
            state_changed: false,
            last_id: 1,
            todos: None,
            conf,
            todo_repository: None,
        }
    }

    pub fn initialize(&mut self) {
        if self.is_initialized {
            return;
        }

        self.todo_repository = Some(TodoRepository::new(PathBuf::from(
            self.conf.db_file_path.as_str(),
        )));
        self.load_todos();
    }

    pub fn save_state(&mut self) {
        self.save_todos();

        self.state_changed = false
    }

    fn load_todos(&mut self) {
        info!("Loading todos from file '{}'.", self.conf.db_file_path);

        self.todos = match self.todo_repository.as_ref().unwrap().load_todos() {
            Ok(todos) => Some(todos),
            Err(e) => panic!("Error loading todos from database: {:?}", e),
        };

        self.last_id = self
            .get_todos()
            .keys()
            .fold(self.last_id, |acc, i| acc.max(*i));
        self.is_initialized = true;

        info!("Loaded {} todos.", self.todos.as_ref().unwrap().len());
    }

    fn save_todos(&mut self) {
        info!("Saving todos to file {}", self.conf.db_file_path);

        match self
            .todo_repository
            .as_ref()
            .unwrap()
            .save_todos(self.get_todos())
        {
            Err(e) => panic!("Failed to save todos to database: {}", e),
            _ => (),
        }
    }

    pub fn get_todos(&self) -> &HashMap<Id, Todo> {
        self.todos.as_ref().unwrap()
    }

    fn get_mut_todos(&mut self) -> &mut HashMap<Id, Todo> {
        self.todos.as_mut().unwrap()
    }

    pub fn add_todo(&mut self, mut todo: Todo) -> Option<&Todo> {
        self.last_id += 1;
        todo.id = self.last_id;

        let new_id = self.last_id;
        self.get_mut_todos().insert(new_id, todo);

        self.state_changed = true;

        self.save_state();

        info!("Added todo {}", self.last_id);

        self.get_todos().get(&self.last_id)
    }

    pub fn rm_todo(&mut self, id: Id) -> Option<Todo> {
        let todo = self.get_mut_todos().remove(&id);

        if todo.is_some() {
            self.save_state();
        }

        info!("Removed todo {}", id);

        todo
    }
}
