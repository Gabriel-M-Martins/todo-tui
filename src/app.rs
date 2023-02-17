use std::io;

pub struct App {
    pub quit: bool,
    pub db: Database,
    pub mode: Mode,
    pub input: String
}

impl App {
    pub fn on_key(&mut self, c: char) {
        let ch = &c.to_ascii_lowercase();
        match ch {
            'q' => self.quit = true,
            'i' => self.mode = Mode::Input,
            _ => {}
        }
    }

    pub fn change_mode(&mut self) {
        match self.mode {
            Mode::Input => self.mode = Mode::Normal,
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App { 
            quit: false, 
            db: Database::load().unwrap(), // todo: remove .unwrap() and deal with error possibility (probably justified to exit program if cannot load nor create new db)
            mode: Mode::Normal,
            input: String::new()
        }
    }
}

pub enum Mode {
    Input,
    Normal,
}

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

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
        todo!()
    }

    pub fn drop(&self) -> Result<(), io::Error> {
        todo!()
    }
}