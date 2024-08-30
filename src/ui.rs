use crate::app::{App, CurrentScreen};
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{BorderType, Clear, List, ListItem, Paragraph};
use ratatui::{prelude::*, widgets::Block};

const ONLINE_USERS_STR: &str = " Online users ";
const BORDER_WIDTH: usize = 1;
pub const USERNAME_WRAP_WIDTH: usize = 4;

impl App {
    pub fn ui(&mut self, frame: &mut Frame)
    where
        Self: Sized,
    {
        let [chat_window, online_users_window] =
            Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
                .areas(frame.area());

        {
            let online_users_block = Block::bordered()
                .style(Style::default())
                .border_type(BorderType::Rounded)
                .title(
                    Title::from(ONLINE_USERS_STR)
                        .position(Position::Top)
                        .alignment(Alignment::Center),
                );

            let mut list_items: Vec<ListItem> = Vec::new();
            for username in &self.online_users {
                list_items.push(ListItem::new(Text::from(format!("> {}", username.clone()))));
            }
            let usernames_list = List::new(list_items).block(online_users_block);
            frame.render_widget(usernames_list, online_users_window);
        }

        let [messages_box, chat_input] =
            Layout::vertical([Constraint::Percentage(100), Constraint::Min(3)]).areas(chat_window);

        self.max_chat_index = self
            .chat_messages
            .1
            .len()
            .checked_sub(messages_box.height as usize - BORDER_WIDTH)
            .unwrap_or(0);
        {
            let chat_input_block = Block::bordered()
                .style(Style::default())
                .border_type(BorderType::Rounded);

            let para = Paragraph::new(self.input_field.clone()).block(chat_input_block);
            frame.render_widget(para, chat_input);
        }

        {
            let mut messages_box_block = Block::bordered()
                .style(Style::default())
                .border_type(BorderType::Rounded);

            if let Some(username) = &self.username {
                messages_box_block = messages_box_block.title(
                    Title::from(username.clone())
                        .alignment(Alignment::Left)
                        .position(Position::Top),
                );
            }

            if let Some(room_name) = &self.room_name {
                messages_box_block = messages_box_block.title(
                    Title::from(format!(" {} ", room_name))
                        .alignment(Alignment::Center)
                        .position(Position::Top),
                );
            }

            let mut messages_list: Vec<ListItem> = Vec::new();
            if self.chat_messages.0 != (messages_box.width as usize).saturating_sub(BORDER_WIDTH) {
                self.create_lines((messages_box.width as usize).saturating_sub(BORDER_WIDTH));
            }

            let start = self.chat_index.clamp(
                0,
                self.chat_messages
                    .1
                    .len()
                    .checked_sub(messages_box.height as usize - BORDER_WIDTH)
                    .unwrap_or(0),
            );
            let end = (start + messages_box.height as usize - BORDER_WIDTH)
                .clamp(start, self.chat_messages.1.len());

            for s in &self.chat_messages.1[start..end] {
                messages_list.push(ListItem::new(Text::from(s.clone())));
            }

            let list = List::new(messages_list).block(messages_box_block);
            frame.render_widget(list, messages_box);
        }

        match self.current_screen {
            CurrentScreen::Enter => {
                let window = centered_rect(50, 50, frame.area());
                let enter_block = Block::bordered()
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black));
                let inner = enter_block.inner(window);
                let [_, username_rect, _, room_rect, _] = Layout::vertical([
                    Constraint::Percentage(23),
                    Constraint::Min(3),
                    Constraint::Percentage(45),
                    Constraint::Min(3),
                    Constraint::Percentage(23),
                ])
                .areas(inner);
                frame.render_widget(enter_block, window);
                let username_block = Block::bordered().border_type(BorderType::Rounded);
                let room_block = Block::bordered().border_type(BorderType::Rounded);
                let username_input =
                    Paragraph::new(self.username_input.as_str()).block(username_block);
                let room_input = Paragraph::new(self.room_input.as_str()).block(room_block);

                frame.render_widget(Clear, inner);
                frame.render_widget(username_input, username_rect);
                frame.render_widget(room_input, room_rect);
            }
            CurrentScreen::Main => {}
            CurrentScreen::Quit => {}
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
