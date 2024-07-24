use ratatui::{crossterm::event::KeyCode, prelude::*, widgets::Widget};

use super::View;

pub struct NewTaskView;

impl View for NewTaskView {
    fn view_event_key(&mut self, key: KeyCode) {}

    fn render_view(&mut self, area: Rect, buf: &mut Buffer) {}
}

impl Widget for NewTaskView {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.render_view(area, buf);
    }
}
