use ratatui::{crossterm::event::KeyCode, prelude::*, widgets::Widget};
use std::{cell::RefCell, rc::Rc};

use super::View;
use crate::{app::ApplicationState, widgets::Input};

pub struct NewTaskView {
    title: Input,
    description: Input,
    app_state: Rc<RefCell<ApplicationState>>,
}

impl NewTaskView {
    pub fn new(app_state: Rc<RefCell<ApplicationState>>) -> Self {
        Self {
            title: Input::new(),
            description: Input::new(),
            app_state,
        }
    }
}

impl View for NewTaskView {
    fn view_event_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('i') => {
                self.title.focused();
            }
            _ => {}
        }
    }

    fn render_view(&mut self, area: Rect, buf: &mut Buffer) {
        self.title.render(area, buf);
    }
}
