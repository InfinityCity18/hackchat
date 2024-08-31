use app::App;
use color_eyre::Result;
use tui::init_panic_hook;

mod app;
mod network;
mod tui;
mod ui;

fn main() -> Result<()> {
    network::udp_manager();
    color_eyre::install()?;
    init_panic_hook();
    let mut terminal = tui::init_tui()?;
    let mut app = App::new();

    app.online_users
        .lock()
        .unwrap()
        .insert("User2137".to_string());
    app.network_messages
        .lock()
        .unwrap()
        .push(("test".to_string(), "hi :3".to_string()));
    app.room_name = Some("bajo jajo".to_string());
    app.username = Some("user2".to_string());
    app.username_input = "testo".to_string();
    app.room_input = "roomnamme".to_string();

    let result = app.run(&mut terminal);
    if let Err(err) = tui::restore_tui() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    result
}
