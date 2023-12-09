use std::io::stdout;

use anyhow::{bail, Context, Result};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub mod tui_main;

pub fn enter_tui() -> Result<()> {
    stdout()
        .execute(EnterAlternateScreen)
        .context("Failed to enter alternative screen mode in terminal.")?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))
        .context("Failed to create Ratatui terminal.")?;

    loop {
        tui_main::draw_terminal(&mut terminal)
            .context("Something went wrong running draw_terminal routine.")?;

        match tui_main::handle_events().context("Something went wrong handling the events") {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => bail!(e),
        };
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
