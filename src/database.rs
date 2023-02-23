use std::io;

pub struct Task {}

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