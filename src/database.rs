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

    pub fn search(&self) -> Option<Task> {
        unimplemented!()
    }

    pub fn drop(&self) -> Result<(), io::Error> {
        unimplemented!()
    }

    pub fn push(&self) {
        unimplemented!()
    }

    pub fn remove(&self, index: i64) {
        unimplemented!()
    }
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
    }
}