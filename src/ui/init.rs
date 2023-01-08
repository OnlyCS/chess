use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{ui::errors::UIError, utils::safe_unwrap::safe_unwrap_err};

pub fn ui_init() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    safe_unwrap_err!(enable_raw_mode(), UIError::CreateFailed);
    let mut stdout = io::stdout();

    safe_unwrap_err!(
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture),
        UIError::CreateFailed
    );

    let backend = CrosstermBackend::new(stdout);
    let terminal = safe_unwrap_err!(Terminal::new(backend), UIError::CreateFailed);

    Ok(terminal)
}
