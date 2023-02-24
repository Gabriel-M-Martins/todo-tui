use std::io;

use crate::database::{Database, InputType, task::Task};

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
        if let Some(cmd) = &self.cmd {
            match cmd {
                Command::New    => {
                    self.db.push(Task::parse(&self.input));
                }
                Command::Delete => {
                    self.db.remove(self.type_check_input());
                }
                Command::Search => {
                    // todo: make it so it searches automatically when writing on input
                }
                Command::Toggle => {
                    // fixme: this feels weird. getting a mutable reference to the task is awkward.
                    if let Some(task) = self.db.search(self.type_check_input()) {
                        task.toggle();
                    }
                }
            }
        }
        
        self.cmd = None;
    }

    fn type_check_input(&self) -> InputType {
        let i = self.input.trim().parse::<i64>();
        if let Ok(i) = i {
            return InputType::Index(i);
        } else {
            return InputType::Name(self.input.to_owned());
        }
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