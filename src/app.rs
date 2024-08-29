pub struct App {
    pub current_screen: CurrentScreen,
    pub username: Option<String>,
    pub room_name: Option<String>,
}

pub enum CurrentScreen {
    Enter,
    Main,
    Quit,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Enter,
            username: None,
            room_name: None,
        }
    }
}
