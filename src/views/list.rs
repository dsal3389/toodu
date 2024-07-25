use ratatui::{
    crossterm::event::KeyCode,
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph},
};
use std::{borrow::BorrowMut, time::Duration};

use super::View;
use crate::widgets::{CenteredText, Notification, NotificationLevel, NotificationStack, TodoList};

pub struct ListView<'a> {
    notification_stack: &'a mut NotificationStack,
    todo_list: &'a mut TodoList,
}

impl<'a> ListView<'a> {
    pub fn new(notification_stack: &'a mut NotificationStack, todo_list: &'a mut TodoList) -> Self {
        Self {
            notification_stack,
            todo_list,
        }
    }

    fn render_todo_item_content(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::TOP).white().on_black();
        match self.todo_list.selected() {
            Some(item) => {
                let inner_block = Block::bordered()
                    .padding(Padding::horizontal(1))
                    .border_style(Style::default().light_blue())
                    .on_black();
                let inner_area = Layout::default()
                    .vertical_margin(1)
                    .horizontal_margin(5)
                    .constraints([Constraint::Min(0)])
                    .split(block.inner(area))[0];

                block.render(area, buf);
                Paragraph::new(item.description().clone())
                    .block(inner_block)
                    .render(inner_area, buf);
            }
            None => {
                CenteredText::new(
                    Text::from("scroll on some tasks to view thier content here").cyan(),
                )
                .block(block)
                .render(area, buf);
            }
        }
    }

    fn render_controls_line(&mut self, area: Rect, buf: &mut Buffer) {
        Line::from(key_spans!(
            "k/j",
            "UP/DN",
            "TAB/Enter",
            "toggle task status",
            "n",
            "new task",
            "d/DEL",
            "delete task",
            "q/esc",
            "quit"
        ))
        .black()
        .on_white()
        .bold()
        .render(area, buf);
    }
}

impl<'a> View for ListView<'a> {
    fn view_event_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.todo_list.next(),
            KeyCode::Char('k') | KeyCode::Up => self.todo_list.prev(),
            KeyCode::Char('d') | KeyCode::Delete => {
                if let Some(item) = self.todo_list.delete_current() {
                    self.notification_stack
                        .borrow_mut()
                        .push_notification(Notification::new(
                            " deleted item ".into(),
                            format!(
                                "deleted item `{}` from todo list with status {}",
                                item.title(),
                                item.status()
                            ),
                            Duration::from_secs(7),
                            NotificationLevel::Warn,
                        ));
                }
            }
            KeyCode::Enter | KeyCode::Tab => self.todo_list.toggle_current_status(),
            _ => {}
        };
    }

    #[inline]
    fn render_view(&mut self, area: Rect, buf: &mut Buffer) {
        let [list_area, content_area, controls_area] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Length(1),
        ])
        .areas(area);

        self.todo_list.render(list_area, buf);
        self.render_todo_item_content(content_area, buf);
        self.render_controls_line(controls_area, buf);
    }
}

impl<'a> Widget for ListView<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.render_view(area, buf);
    }
}
