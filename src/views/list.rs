use ratatui::{
    crossterm::event::KeyCode,
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph},
};
use std::{cell::RefCell, rc::Rc, time::Duration};

use super::View;
use crate::{
    app::ApplicationState,
    widgets::{CenteredText, Notification, NotificationLevel},
};

pub struct ListView {
    app_state: Rc<RefCell<ApplicationState>>,
}

impl ListView {
    pub fn new(app_state: Rc<RefCell<ApplicationState>>) -> Self {
        Self { app_state }
    }

    fn render_todo_item_content(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::TOP).white().on_black();
        match self.app_state.borrow().todo_list.selected() {
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

    fn render_controls_line(&self, area: Rect, buf: &mut Buffer) {
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

impl View for ListView {
    fn view_event_key(&mut self, key: KeyCode) {
        let mut state = self.app_state.borrow_mut();

        match key {
            KeyCode::Char('j') | KeyCode::Down => state.todo_list.next(),
            KeyCode::Char('k') | KeyCode::Up => state.todo_list.prev(),
            KeyCode::Char('d') | KeyCode::Delete => {
                if let Some(item) = state.todo_list.delete_current() {
                    state.notifications.push_notification(Notification::new(
                        " deleted item ".into(),
                        format!(
                            "deleted item `{}` from todo list with status {}",
                            item.title(),
                            item.status()
                        ),
                        Duration::from_secs(5),
                        NotificationLevel::Warn,
                    ));
                }
            }
            KeyCode::Enter | KeyCode::Tab => state.todo_list.toggle_current_status(),
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

        self.render_controls_line(controls_area, buf);
        self.render_todo_item_content(content_area, buf);
        self.app_state.borrow_mut().todo_list.render(list_area, buf);
    }
}
