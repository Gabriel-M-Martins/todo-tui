use strum::VariantNames;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, ListItem, List, Tabs, Paragraph, Wrap},
    Frame, style::{Style, Modifier, Color}, text::{Spans, Text, Span}, symbols::DOT,
};

use crate::app::{App, Mode};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 8),
                Constraint::Ratio(6, 8),
                Constraint::Ratio(1, 8),
            ]
            .as_ref(),
        )
        .split(f.size());

    // -------------------------------------------------------------------------- options block
    
    let tabs_titles  = Options::VARIANTS.iter().cloned().map(|f| {
        let (first, rest) = f.split_at(1);
        Spans::from(vec![
            Span::styled(first, Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled(rest, Style::default())
        ])
    }).collect();

    let tabs =  Tabs::new(tabs_titles)
        .block(Block::default().title("Options").borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(Style::default())
        .divider(DOT);

    f.render_widget(tabs, chunks[0]);   
    
    // -------------------------------------------------------------------------- middle block
    
    let tasks = Block::default().title("Tasks").borders(Borders::ALL);
    f.render_widget(tasks, chunks[1]);

    // -------------------------------------------------------------------------- input block
    
    let mut input_text = Spans::default();
    match app.mode {
        Mode::Input => { 
            input_text = Spans::from(vec![
            Span::raw(&app.input),
            Span::styled("_", Style::default().add_modifier(Modifier::RAPID_BLINK)),
            ]);
        }
        _ => {
            input_text = Spans::from(vec![Span::raw(&app.input)]);
        }
    }

    let input = Paragraph::new(input_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(input, chunks[2]);

}

#[derive(strum::EnumVariantNames)]
pub enum Options {
    All,
    New,
    Delete,
    Search,
    Toggle,
    Exit,
}