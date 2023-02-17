pub struct App {
    pub quit: bool,
}

impl App {
    pub fn default() -> Self {
        App { quit: false }
    }

    pub fn on_key(&mut self, c: char) {
        let ch = &c.to_ascii_lowercase();
        match ch {
            'q' => self.quit = true,
            _ => {}
        }
    }
}
