use anyhow::Context;
use ratatui::{
    crossterm::event::{poll, read, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::Widget,
    Terminal,
};
use std::time::Duration;

use crate::{
    views::{ListView, NewTaskView, View},
    widgets::{NotificationStack, TodoItem, TodoList},
};

#[derive(Default, PartialEq)]
pub enum ApplicationState {
    #[default]
    Running,
    Exiting,
}

#[derive(Default)]
pub enum ApplicationView {
    #[default]
    TodoListView,
    TodoItemAdd,
}

#[derive(Default)]
pub struct Application {
    view: ApplicationView,
    running_state: ApplicationState,
    notifications: NotificationStack,
    todo_list: TodoList,
}

impl Application {
    pub fn load_data(&mut self) {
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
        while self.running_state != ApplicationState::Exiting {
            terminal
                .draw(|f| f.render_widget(&mut *self, f.size()))
                .context("couldn't draw new frame to terminal screen")?;
            self.wait_for_key_event()?;
        }
        Ok(())
    }

    fn wait_for_key_event(&mut self) -> anyhow::Result<()> {
        if poll(Duration::from_secs(0))? {
            if let Event::Key(key_event) = read().context("couldn't read input key event")? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event.code);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyCode) {
        match self.view {
            ApplicationView::TodoListView => match key {
                KeyCode::Char('n') => self.view = ApplicationView::TodoItemAdd,
                KeyCode::Char('q') | KeyCode::Esc => self.running_state = ApplicationState::Exiting,
                _ => {
                    ListView::new(&mut self.notifications, &mut self.todo_list).view_event_key(key);
                }
            },
            ApplicationView::TodoItemAdd => {}
        }
    }
}

impl Widget for &mut Application {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.view {
            ApplicationView::TodoListView => {
                ListView::new(&mut self.notifications, &mut self.todo_list).render(area, buf)
            }
            ApplicationView::TodoItemAdd => NewTaskView.render(area, buf),
        }

        if !self.notifications.is_empty() {
            self.notifications.render(area, buf);
        }
    }
}
