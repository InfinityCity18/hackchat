use color_eyre::{eyre::Ok, Result};
use tui::init_panic_hook;

mod app;
mod tui;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    init_panic_hook();
    let terminal = tui::init_tui()?;
    if let Err(err) = tui::restore_tui() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    Ok(())
}
