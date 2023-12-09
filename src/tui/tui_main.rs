use std::io::Stdout;

use anyhow::{Context, Ok, Result};
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend, style::Stylize, widgets::Paragraph, CompletedFrame, Terminal,
};

pub fn draw_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<CompletedFrame> {
    terminal
        .draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Bucketinator! (press 'q' to quit.)")
                    .white()
                    .on_blue(),
                area,
            );
        })
        .context("Failed to draw terminal.")
}

pub fn handle_events() -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(false);
            }
        }
    }

    return Ok(true);
}
