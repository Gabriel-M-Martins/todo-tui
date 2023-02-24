use std::io;

use self::task::Task;

pub struct Database {
    tasks: Vec<Task>
}

impl Database {
    pub fn load() -> Result<Self, io::Error> {
        Ok(
            Database { tasks: vec![] }
        )
    }

    pub fn search(&self, method: InputType) -> Option<&mut Task> {
        match method {
            InputType::Name(n) => {
                self.search_name(n)
            }
            InputType::Index(i) => {
                self.search_id(i)
            }
        }
    }

    fn search_id(&self, index: i64) -> Option<&mut Task> {
        unimplemented!()
    }

    fn search_name(&self, name: impl Into<String>) -> Option<&mut Task> {
        // todo: use some crate to have fuzzy search
        unimplemented!()
    }

    pub fn drop(&self) -> Result<(), io::Error> {
        unimplemented!()
    }

    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove(&mut self, method: InputType) {
        match method {
            InputType::Name(n) => {
                self.remove_name(n);
            }
            InputType::Index(i) => {
                self.remove_id(i);
            }
        }
    }

    fn remove_id(&self, index: i64) {
        unimplemented!()
    }

    fn remove_name(&self, name: impl Into<String>){
        // todo: use some crate to have fuzzy search
        unimplemented!()
    }
}

/// If the command used (search, remove and toggle) is by name or index.
pub enum InputType {
    Name(String),
    Index(i64)
}

pub mod task {
    use chrono::{DateTime, Local};

    pub struct Task {
        pub completed: bool,
        pub priority: Option<char>,
        pub dates: Option<Dates>,
        pub desc: Description
    }

    pub struct Description {
        pub txt: String,
        pub tags: Vec<Tags>
    }

    pub enum Tags {
        Project(String),
        Context(String),
        Special((String, String))
    }

    pub struct Dates {
        pub creation: DateTime<Local>,
        pub completed: Option<DateTime<Local>>,
    }

    impl Default for Description {
        fn default() -> Self {
            Description { txt: String::new(), tags: vec![] }
        }
    }

    impl Default for Task {
        fn default() -> Self {
            Task {
                completed: false,
                priority: None,
                dates: None,
                desc: Description::default()
            }
        }
    }

    impl Task {
        pub fn parse(txt: impl Into<String>) -> Self {
            unimplemented!()
        }

        pub fn toggle(&mut self) {
            self.completed = !self.completed;
        }
    }
}