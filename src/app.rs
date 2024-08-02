use anyhow::Context;
use ratatui::{
    crossterm::event::{poll, read, Event, KeyCode, KeyEventKind},
    prelude::*,
    widgets::Widget,
    Terminal,
};
use std::{cell::RefCell, rc::Rc, time::Duration};

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
    state: Rc<RefCell<ApplicationState>>,
    current_view: Option<Box<dyn View>>,
}

impl Application {
    pub fn init(&mut self) {
        self.load_data();
        self.current_view = Some(Box::new(ListView::new(Rc::clone(&self.state))));
    }

    pub fn load_data(&self) {
        let mut state = self.state.borrow_mut();
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
        state
            .todo_list
            .add(TodoItem::new("hello world".into(), "description 1".into()));
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
        while self.state.borrow().running_state != ApplicationRunningState::Exiting {
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
            KeyCode::Char('n') => (*self.state.borrow_mut()).view = ApplicationView::TodoItemAdd,
            KeyCode::Char('q') | KeyCode::Esc => {
                self.state.borrow_mut().running_state = ApplicationRunningState::Exiting
            }
            _ => match &mut self.current_view {
                Some(v) => {
                    v.view_event_key(key);
                }
                None => {}
            },
        }
    }
}

impl Widget for &mut Application {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match &mut self.current_view {
            Some(v) => v.render_view(area, buf),
            None => {}
        }

        if !self.state.borrow().notifications.is_empty() {
            self.state.borrow_mut().notifications.render(area, buf);
        }
    }
}
