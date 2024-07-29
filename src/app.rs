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
pub enum ApplicationRunningState {
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
pub struct ApplicationState {
    pub view: ApplicationView,
    pub running_state: ApplicationRunningState,
    pub notifications: NotificationStack,
    pub todo_list: TodoList,
}

#[derive(Default)]
pub struct Application {
    state: ApplicationState,
}

impl Application {
    pub fn load_data(&mut self) {
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        self.state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
        while self.state.running_state != ApplicationRunningState::Exiting {
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
        match key {
            KeyCode::Char('n') => self.state.view = ApplicationView::TodoItemAdd,
            KeyCode::Char('q') | KeyCode::Esc => {
                self.state.running_state = ApplicationRunningState::Exiting
            }
            _ => match self.state.view {
                ApplicationView::TodoListView => ListView::new(&mut self.state).view_event_key(key),
                ApplicationView::TodoItemAdd => {
                    NewTaskView::new(&mut self.state).view_event_key(key)
                }
            },
        }
    }
}

impl Widget for &mut Application {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.state.view {
            ApplicationView::TodoListView => ListView::new(&mut self.state).render(area, buf),
            ApplicationView::TodoItemAdd => NewTaskView::new(&mut self.state).render(area, buf),
        }

        if !self.state.notifications.is_empty() {
            self.state.notifications.render(area, buf);
        }
    }
}
