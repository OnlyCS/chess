use std::{error::Error, io::Stdout};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{ui::errors::UIError, utils::safe_unwrap::safe_unwrap_err};

pub fn ui_destroy(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    safe_unwrap_err!(disable_raw_mode(), UIError::DestroyFailed);

    safe_unwrap_err!(
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ),
        UIError::DestroyFailed
    );

    safe_unwrap_err!(terminal.show_cursor(), UIError::DestroyFailed);

    Ok(())
}
