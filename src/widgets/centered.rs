use ratatui::{
    prelude::*,
    widgets::{Block, Widget},
};

pub struct CenteredText<'a> {
    text: Text<'a>,
    block: Option<Block<'a>>,
}

impl<'a> CenteredText<'a> {
    pub fn new(text: Text<'a>) -> Self {
        Self { text, block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for CenteredText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let text_height: u16 = self.text.height() as u16;
        let center = Layout::vertical([
            Constraint::Length((area.height - text_height) / 2),
            Constraint::Length(text_height),
            Constraint::Length((area.height - text_height) / 2),
        ])
        .split(self.block.inner_if_some(area))[1];

        self.block.render(area, buf);
        self.text.centered().render(center, buf);
    }
}
