use color_eyre::eyre::{Ok, Result};
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::widgets::Widget;
use ratatui::Frame;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::collections::HashSet;
use std::io::Stdout;

pub struct App {
    pub current_screen: CurrentScreen,
    pub username: Option<String>,
    pub room_name: Option<String>,
    pub input_field: String,
    pub input_index: usize,
    pub network_messages: Vec<(String, String)>,
    pub chat_messages: (usize, Vec<String>),
    pub chat_index: usize,
    pub exit: bool,
    pub online_users: HashSet<String>,
    pub follow_chat: bool,
}

pub enum CurrentScreen {
    Enter,
    Main,
    Quit,
}

pub enum Mode {
    Main,
    Searching,
    Inputing,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Enter,
            username: None,
            room_name: None,
            input_field: String::new(),
            input_index: 0,
            network_messages: Vec::new(),
            chat_messages: (0, Vec::new()),
            chat_index: 0,
            exit: false,
            online_users: HashSet::new(),
            follow_chat: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.exit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.input_index.saturating_sub(1);
        self.input_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.input_index.saturating_add(1);
        self.input_index = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_field.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.input_index = 0;
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input_field.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input_field
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.input_index)
            .unwrap_or(self.input_field.len())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn create_lines(&mut self, window_width: usize) {
        let mut lines: Vec<String> = Vec::new();

        let mut i = 1;
        for (username, msg) in &self.network_messages {
            let formated_msg = format!(" |{username}| {msg}");

            let mut line = String::new();
            let mut m = 0;
            for c in formated_msg.chars() {
                if m % window_width == 0 {
                    lines.push(line.clone());
                    line.clear();
                    line += " ";
                    line += &i.to_string();
                    m += line.len();
                    i += 1;
                }
                line.push(c);
                m += 1;
            }

            if line.len() > 0 {
                lines.push(line)
            }
        }

        self.chat_messages = (window_width, lines);
    }
}
