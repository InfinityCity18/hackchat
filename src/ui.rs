use crate::app::App;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{BorderType, List, ListItem, Paragraph};
use ratatui::{prelude::*, widgets::Block};

const ONLINE_USERS_STR: &str = " Online users ";
const BORDER_WIDTH: usize = 1;
const USERNAME_WRAP_WIDTH: usize = 4;

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [chat_window, online_users_window] =
            Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
                .areas(area);

        //online users window
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
            <List as Widget>::render(usernames_list, online_users_window, buf);
        }

        let [messages_box, chat_input] =
            Layout::vertical([Constraint::Percentage(100), Constraint::Min(3)]).areas(chat_window);

        {
            let chat_input_block = Block::bordered()
                .style(Style::default())
                .border_type(BorderType::Rounded);

            let para = Paragraph::new(self.input_field.clone()).block(chat_input_block);
            para.render(chat_input, buf);
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
                    Title::from(room_name.clone())
                        .alignment(Alignment::Left)
                        .position(Position::Top),
                );
            }

            let mut messages_list: Vec<ListItem> = Vec::new();

            /*
            let chat_index= if self.chat_index >
            for a in self.chat_messages {
            let prefix_len = (1 + i/10) + USERNAME_WRAP_WIDTH
            }
            */
        }
    }
}
