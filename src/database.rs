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
    use std::io;

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
            let txt: String = txt.into();
            
            let task = Task::default();

            if txt.starts_with('x') {}
            
            unimplemented!()
        }

        pub fn toggle(mut self) -> Self {
            self.completed = !self.completed;
            return self;
        }

        pub fn priority(mut self, c: char) -> Self {
            self.priority = Some(c);
            return self;
        }

        pub fn creation_date(mut self, date: DateTime<Local>) -> Self {
            self.dates = Some(Dates { creation: date, completed: None });
            return self;
        }

        pub fn completion_date(mut self, date: DateTime<Local>) -> Result<Self, io::ErrorKind> {
            match self.dates {
                Some(dates) => {
                    self.dates = Some(Dates { creation: dates.creation, completed: Some(date) });
                    return Ok(self);
                }
                None => {
                    unimplemented!()
                }
            }
        }

        pub fn description(mut self, txt: impl Into<String>, tags: Option<Vec<Tags>>) -> Self {
            match tags {
                Some(tags) => {
                    self.desc = Description { txt: txt.into(), tags };
                    return self;
                },
                None => {
                    self.desc = Description { txt: txt.into(), tags: vec![] };
                    return self;
                }
            }
        }
    }
}