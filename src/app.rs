use color_eyre::eyre::{Ok, Result};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Position;
use ratatui::prelude::Rect;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::collections::HashSet;
use std::io::Stdout;

pub struct App {
    pub current_screen: CurrentScreen,
    pub mode: Option<Mode>,
    pub username: Option<String>,
    pub room_name: Option<String>,
    pub chat_input: String,
    pub chat_input_index: usize,
    pub network_messages: Vec<(String, String)>,
    pub chat_messages: (usize, Vec<String>),
    pub chat_index: usize,
    pub max_chat_index: usize,
    pub exit: bool,
    pub online_users: HashSet<String>,
    pub follow_chat: bool,
    pub inserting: Inserting,
    pub username_input: String,
    pub room_input: String,
    pub username_index: usize,
    pub room_index: usize,
}

pub enum CurrentScreen {
    Login,
    Main,
    Quit,
}

pub enum Mode {
    Main,
    Searching,
    Inputing,
}

#[derive(Clone, Copy)]
pub enum Inserting {
    Username,
    Room,
    Chat,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Login,
            mode: None,
            username: None,
            room_name: None,
            chat_input: String::new(),
            chat_input_index: 0,
            network_messages: Vec::new(),
            chat_messages: (0, Vec::new()),
            chat_index: 0,
            max_chat_index: 0,
            exit: false,
            online_users: HashSet::new(),
            follow_chat: false,
            inserting: Inserting::Username,
            username_input: String::new(),
            room_input: String::new(),
            username_index: 0,
            room_index: 0,
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
            Event::Key(key) if key.kind == KeyEventKind::Press => match self.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Esc => self.current_screen = CurrentScreen::Quit,
                    _ => {}
                },
                CurrentScreen::Quit => match key.code {
                    KeyCode::Char('y') => self.exit(),
                    KeyCode::Char('n') => self.current_screen = CurrentScreen::Main,
                    KeyCode::Esc => self.current_screen = CurrentScreen::Main,
                    _ => {}
                },
                CurrentScreen::Login => match key.code {
                    KeyCode::Esc => self.exit(),
                    KeyCode::Tab => self.switch_inserting_mode(),
                    KeyCode::Char(c) => match self.inserting {
                        Inserting::Username => self.enter_char(c, self.inserting),
                        Inserting::Room => self.enter_char(c, self.inserting),
                        Inserting::Chat => {}
                    },
                    KeyCode::Backspace => match self.inserting {
                        Inserting::Username => self.delete_char(self.inserting),
                        Inserting::Room => self.delete_char(self.inserting),
                        Inserting::Chat => panic!("inserting chat while in login screen"),
                    },
                    KeyCode::Left => match self.inserting {
                        Inserting::Username => self.move_cursor_left(self.inserting),
                        Inserting::Room => self.move_cursor_left(self.inserting),
                        Inserting::Chat => panic!("inserting chat while in login screen"),
                    },
                    KeyCode::Right => match self.inserting {
                        Inserting::Username => self.move_cursor_right(self.inserting),
                        Inserting::Room => self.move_cursor_right(self.inserting),
                        Inserting::Chat => panic!("inserting chat while in login screen"),
                    },
                    KeyCode::Enter => self.submit_login(),

                    _ => {}
                },
            },
            _ => {}
        }
        Ok(())
    }

    fn switch_inserting_mode(&mut self) {
        match &self.inserting {
            Inserting::Username => self.inserting = Inserting::Room,
            Inserting::Room => self.inserting = Inserting::Username,
            Inserting::Chat => {}
        }
    }

    fn submit_login(&mut self) {
        self.username = Some(self.username_input.clone());
        self.room_name = Some(self.room_input.clone());
        self.current_screen = CurrentScreen::Main;
        self.mode = Some(Mode::Main);
    }

    fn scroll_up(&mut self) {
        self.chat_index = self.chat_index.saturating_sub(1);
    }

    fn scroll_down(&mut self) {
        self.chat_index = (self.chat_index + 1).clamp(0, self.max_chat_index);
    }

    fn move_cursor_left(&mut self, inserting: Inserting) {
        match inserting {
            Inserting::Room => {
                let cursor_moved_left = self.room_index.saturating_sub(1);
                self.room_index = self.clamp_cursor(cursor_moved_left, inserting);
            }
            Inserting::Username => {
                let cursor_moved_left = self.username_index.saturating_sub(1);
                self.username_index = self.clamp_cursor(cursor_moved_left, inserting);
            }
            Inserting::Chat => {
                let cursor_moved_left = self.chat_input_index.saturating_sub(1);
                self.chat_input_index = self.clamp_cursor(cursor_moved_left, inserting);
            }
        }
    }

    fn move_cursor_right(&mut self, inserting: Inserting) {
        match inserting {
            Inserting::Room => {
                let cursor_moved_right = self.room_index.saturating_add(1);
                self.room_index = self.clamp_cursor(cursor_moved_right, inserting);
            }
            Inserting::Username => {
                let cursor_moved_right = self.username_index.saturating_add(1);
                self.username_index = self.clamp_cursor(cursor_moved_right, inserting);
            }
            Inserting::Chat => {
                let cursor_moved_right = self.chat_input_index.saturating_add(1);
                self.chat_input_index = self.clamp_cursor(cursor_moved_right, inserting);
            }
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize, inserting: Inserting) -> usize {
        let count;
        match inserting {
            Inserting::Username => count = self.username_input.chars().count(),
            Inserting::Room => count = self.room_input.chars().count(),
            Inserting::Chat => count = self.chat_input.chars().count(),
        }
        new_cursor_pos.clamp(0, count)
    }

    pub fn cursor_pos(&self, input_area: Rect, inserting: Inserting) -> Position {
        match inserting {
            Inserting::Username => Position::new(
                input_area.x + self.username_index as u16 + 1,
                input_area.y + 1,
            ),
            Inserting::Room => {
                Position::new(input_area.x + self.room_index as u16 + 1, input_area.y + 1)
            }
            Inserting::Chat => Position::new(
                input_area.x + self.chat_input_index as u16 + 1,
                input_area.y + 1,
            ),
        }
    }

    fn reset_cursor(&mut self, inserting: Inserting) {
        match inserting {
            Inserting::Room => self.room_index = 0,
            Inserting::Username => self.username_index = 0,
            Inserting::Chat => self.chat_input_index = 0,
        }
    }

    fn enter_char(&mut self, new_char: char, inserting: Inserting) {
        let index = self.byte_index(inserting);
        match inserting {
            Inserting::Room => self.room_input.insert(index, new_char),
            Inserting::Chat => self.chat_input.insert(index, new_char),
            Inserting::Username => self.username_input.insert(index, new_char),
        }
        self.move_cursor_right(inserting);
    }

    fn delete_char(&mut self, inserting: Inserting) {
        let is_not_cursor_leftmost = match inserting {
            Inserting::Room => self.room_index != 0,
            Inserting::Username => self.username_index != 0,
            Inserting::Chat => self.chat_input_index != 0,
        };
        if is_not_cursor_leftmost {
            match inserting {
                Inserting::Room => {
                    let current_index = self.room_index;
                    let from_left_to_current_index = current_index - 1;
                    let before_char_to_delete =
                        self.room_input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.room_input.chars().skip(current_index);
                    self.room_input = before_char_to_delete.chain(after_char_to_delete).collect();
                    self.move_cursor_left(inserting);
                }
                Inserting::Username => {
                    let current_index = self.username_index;
                    let from_left_to_current_index = current_index - 1;
                    let before_char_to_delete =
                        self.username_input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.username_input.chars().skip(current_index);
                    self.username_input =
                        before_char_to_delete.chain(after_char_to_delete).collect();
                    self.move_cursor_left(inserting);
                }
                Inserting::Chat => {
                    let current_index = self.chat_input_index;
                    let from_left_to_current_index = current_index - 1;
                    let before_char_to_delete =
                        self.chat_input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.chat_input.chars().skip(current_index);
                    self.chat_input = before_char_to_delete.chain(after_char_to_delete).collect();
                    self.move_cursor_left(inserting);
                }
            }
        }
    }

    fn byte_index(&self, inserting: Inserting) -> usize {
        match inserting {
            Inserting::Room => self
                .room_input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.room_index)
                .unwrap_or(self.room_input.len()),
            Inserting::Chat => self
                .chat_input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.chat_input_index)
                .unwrap_or(self.chat_input.len()),
            Inserting::Username => self
                .username_input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.username_index)
                .unwrap_or(self.username_input.len()),
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn create_lines(&mut self, window_width: usize) {
        let mut lines: Vec<String> = Vec::new();

        let mut i = 1;
        for (username, msg) in &self.network_messages {
            let formated_msg = format!("|{username}| {msg}");

            let mut line = String::new();
            let mut m = 0;
            for c in formated_msg.chars() {
                if m % window_width == 0 {
                    if line.len() > 0 {
                        lines.push(line.clone());
                    }
                    line.clear();
                    line += " ";
                    line += &i.to_string();
                    line += " ";
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

    pub fn add_message_to_network_and_chat(&mut self, username: String, msg: String) {
        self.network_messages.push((username.clone(), msg.clone()));
        let mut lines = Vec::new();
        let mut i = 1;
        let formated_msg = format!("|{username}| {msg}");

        let mut line = String::new();
        let mut m = 0;
        for c in formated_msg.chars() {
            if m % self.chat_messages.0 == 0 {
                if line.len() > 0 {
                    lines.push(line.clone());
                }
                line.clear();
                line += " ";
                line += &i.to_string();
                line += " ";
                m += line.len();
                i += 1;
            }
            line.push(c);
            m += 1;
        }

        if line.len() > 0 {
            lines.push(line)
        }

        for line in lines {
            self.chat_messages.1.push(line);
        }
    }
}
