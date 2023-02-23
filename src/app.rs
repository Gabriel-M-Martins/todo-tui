use std::io;

use crate::database::Database;

pub struct App {
    pub quit: bool,
    pub db: Database,
    pub mode: Mode,
    pub cmd: Option<Command>,
    pub input: String
}

impl App {
    pub fn on_key(&mut self, c: char) {
        let ch = &c.to_ascii_lowercase();
        match ch {
            'e' => self.quit = true,
            'i' => self.mode = Mode::Input,
            'a' => {},
            'n' => self.cmd = Some(Command::New),
            'd' => self.cmd = Some(Command::Delete),
            's' => self.cmd = Some(Command::Search),
            't' => self.cmd = Some(Command::Toggle),
            _ => {}
        }
    }

    pub fn change_mode(&mut self) {
        match self.mode {
            Mode::Input => self.mode = Mode::Normal,
            _ => {}
        }
    }

    pub fn execute_cmd(&mut self) {
        
        
        self.cmd = None;
        unimplemented!()
    }
}

impl Default for App {
    fn default() -> Self {
        App { 
            quit: false, 
            db: Database::load().unwrap(), // todo: remove .unwrap() and deal with error possibility (probably justified to exit program if cannot load nor create new db)
            mode: Mode::Normal,
            cmd: None,
            input: String::new()
        }
    }
}

pub enum Mode {
    Input,
    Normal,
}

pub enum Command {
    New,
    Delete,
    Search,
    Toggle
}