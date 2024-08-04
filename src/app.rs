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

#[derive(Default, PartialEq)]
pub enum ApplicationMode {
    #[default]
    Normal,
    Writing,
}

#[derive(Default)]
pub struct ApplicationState {
    pub mode: ApplicationMode,
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
        state.todo_list.add(TodoItem::new(
            "hello world".into(),
            "simple todo task example".into(),
        ));
        state
            .todo_list
            .add(TodoItem::new("sleep".into(), "sleep or whatever".into()));
        state.todo_list.add(TodoItem::new(
            "buy that X item".into(),
            "Go to the mall and buy X".into(),
        ));
        state
            .todo_list
            .add(TodoItem::new("test".into(), "test the item".into()));
        state.todo_list.add(TodoItem::new(
            "call X".into(),
            "number: 123-456-7890".into(),
        ));
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
        // lathough it would be logical to first
        // match the `self.state.mode` and based on the mode go to different
        // handlers like so
        //
        // ```
        // match self.state.borrow().mode {
        //     ApplicationMode::Normal => {
        //         // handle `n`, `q`, `l`
        //     },
        //     ApplicationMode::Writing => {
        //         // handle other keys
        //     }
        // }
        // ```
        //
        // it is not possible since we do `self.state.borrow()` and in the key events we need
        // to mutate the state, like in `q` we need to call `borrow_mut` and mutate the
        // `running_state`, but we can't `borrow_mut` because our `self.state.borrow` is still
        // alive and will crash the program, thats why I have this ugly ass solution with if
        // matches
        match key {
            KeyCode::Char('n') if self.state.borrow().mode == ApplicationMode::Normal => {
                self.current_view = Some(Box::new(NewTaskView::new(Rc::clone(&self.state))));
            }
            KeyCode::Char('l') if self.state.borrow().mode == ApplicationMode::Normal => {
                self.current_view = Some(Box::new(ListView::new(Rc::clone(&self.state))));
            }
            KeyCode::Char('q') | KeyCode::Esc
                if self.state.borrow().mode == ApplicationMode::Normal =>
            {
                self.state.borrow_mut().running_state = ApplicationRunningState::Exiting
            }
            _ => match &mut self.current_view {
                Some(v) => {
                    v.view_event_key(key);
                }
                _ => {}
            },
        };
    }
}

impl Widget for &mut Application {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match &mut self.current_view {
            Some(v) => v.render_view(area, buf),
            None => panic!("application `run` is called before setting the `current_view`"),
        }

        if !self.state.borrow().notifications.is_empty() {
            self.state.borrow_mut().notifications.render(area, buf);
        }
    }
}
