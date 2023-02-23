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
            'a' => { self.input = String::new() },
            'n' => { self.cmd = Some(Command::New);    self.enter_input_mode() },
            'd' => { self.cmd = Some(Command::Delete); self.enter_input_mode() },
            's' => { self.cmd = Some(Command::Search); self.enter_input_mode() },
            't' => { self.cmd = Some(Command::Toggle); self.enter_input_mode() },
            _ => {}
        }
    }

    pub fn change_mode(&mut self) {
        match self.mode {
            Mode::Input => self.mode = Mode::Normal,
            _ => {}
        }
    }

    pub fn enter_input_mode(&mut self) {
        self.input = String::new();
        self.mode = Mode::Input;
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