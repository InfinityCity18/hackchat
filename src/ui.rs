use crate::app::App;
use ratatui::widgets::{BorderType, List, ListItem};
use ratatui::{prelude::*, widgets::Block};

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [chat_window, online_users_window] =
            Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
                .areas(area);

        {
            let online_users_block = Block::default()
                .style(Style::default())
                .border_type(BorderType::Rounded);

            let mut list_items: Vec<ListItem> = Vec::new();
            for username in &self.online_users {
                list_items.push(ListItem::new(username.clone()));
            }
            let usernames_list = List::new(list_items).block(online_users_block);
            usernames_list.render(online_users_window, buf);
        }
    }
}
