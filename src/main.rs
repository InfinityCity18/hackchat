use app::App;
use color_eyre::Result;
use tui::init_panic_hook;

mod app;
mod tui;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    init_panic_hook();
    let mut terminal = tui::init_tui()?;
    let mut app = App::new();

    app.online_users.insert("User2137".to_string());

    let result = app.run(&mut terminal);
    if let Err(err) = tui::restore_tui() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    result
}
