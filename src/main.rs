mod app;
mod views;
mod widgets;
use app::Application;

mod tui {
    use anyhow::Context;
    use crossterm::ExecutableCommand;
    use ratatui::{
        backend::{Backend, CrosstermBackend},
        crossterm::terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        Terminal,
    };
    use std::io::stdout;

    pub fn load_terminal() -> anyhow::Result<Terminal<impl Backend>> {
        stdout()
            .execute(EnterAlternateScreen)
            .context("couldn't enter alternated screen")?;
        enable_raw_mode().context("couldn't enter terminal raw mode")?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> anyhow::Result<()> {
        disable_raw_mode().context("couldn't disable raw mode")?;
        stdout()
            .execute(LeaveAlternateScreen)
            .context("couldn't leave alternated screen")?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let terminal = tui::load_terminal()?;
    let mut application = Application::default();

    application.init();
    application.run(terminal)?;
    tui::restore_terminal()?;
    Ok(())
}
