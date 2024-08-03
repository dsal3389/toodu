use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph, Widget},
};

#[derive(Default)]
pub struct Input {
    value: String,
    cursor_index: usize,
    focused: bool,
}

impl Input {
    pub fn focused(&self) -> bool {
        self.focused
    }
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let box_borders = if self.focused {
            Style::new().light_blue()
        } else {
            Style::new().white()
        };

        Paragraph::new(self.value.clone())
            .block(
                Block::bordered()
                    .title("input")
                    .padding(Padding::horizontal(2))
                    .border_style(box_borders),
            )
            .render(area, buf);
    }
}
