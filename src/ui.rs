use strum::VariantNames;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, ListItem, List, Tabs, Paragraph, Wrap},
    Frame, style::{Style, Modifier, Color}, text::Spans, symbols::DOT,
};

use crate::app::App;

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


    let tabs_titles  = Options::VARIANTS.iter().cloned().map(Spans::from).collect();
    let tabs =  Tabs::new(tabs_titles)
        .block(Block::default().title("Options").borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(DOT);

    f.render_widget(tabs, chunks[0]);   
    
    let block = Block::default().title("Tasks").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    // let input_block = Block::default().title("Input").borders(Borders::ALL); 
    // f.render_widget(input_block, chunks[2]);

    let input_text = Paragraph::new(app.input.clone())
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(input_text, chunks[2]);

}

#[derive(strum::EnumVariantNames)]
pub enum Options {
    All,
    Search,
    Exit,
}