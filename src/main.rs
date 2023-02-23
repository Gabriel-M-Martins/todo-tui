#![allow(unused)]

use std::{
    io,
    time::{Duration, Instant},
};

use tui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod app;
mod ui;
mod database;

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    let mut app = app::App::default();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let time_out = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(time_out)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            match app.mode {
                                app::Mode::Normal => app.on_key(c),
                                app::Mode::Input => app.input.push(c),
                            }
                        }
                        KeyCode::Backspace => { app.input.pop(); }
                        KeyCode::Esc => app.change_mode(),
                        KeyCode::Enter => app.execute_cmd(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            //app.on_tick()
            last_tick = Instant::now();
        }

        if app.quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
