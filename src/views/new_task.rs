use ratatui::{crossterm::event::KeyCode, prelude::*, widgets::Widget};

use super::View;
use crate::{app::ApplicationState, widgets::Input};

pub struct NewTaskView<'a> {
    title: Input,
    description: Input,
    app_state: &'a ApplicationState,
}

impl<'a> NewTaskView<'a> {
    pub fn new(app_state: &'a mut ApplicationState) -> Self {
        Self {
            title: Input::new(),
            description: Input::new(),
            app_state,
        }
    }
}

impl<'a> View for NewTaskView<'a> {
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

impl<'a> Widget for NewTaskView<'a> {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.render_view(area, buf);
    }
}
