use ratatui::{crossterm::event::KeyCode, prelude::*, widgets::Widget};
use std::{cell::RefCell, rc::Rc};

use super::View;
use crate::{
    app::{ApplicationMode, ApplicationState},
    widgets::Input,
};

pub struct NewTaskView {
    title: Input,
    description: Input,
    app_state: Rc<RefCell<ApplicationState>>,
}

impl NewTaskView {
    pub fn new(app_state: Rc<RefCell<ApplicationState>>) -> Self {
        Self {
            title: Input::default(),
            description: Input::default(),
            app_state,
        }
    }
}

impl View for NewTaskView {
    fn view_event_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.title.toggle_focuse();
                self.app_state.borrow_mut().mode = ApplicationMode::Writing;
            }
            _ => {}
        }
    }

    fn render_view(&mut self, area: Rect, buf: &mut Buffer) {
        let [title_area, description_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        self.title.render(title_area, buf);
        self.description.render(description_area, buf);
    }
}
