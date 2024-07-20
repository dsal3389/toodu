use anyhow::Context;
use ratatui::{
    prelude::*,
    widgets::{Block, Widget},
};

pub struct CenteredText<'a>(Text<'a>, Option<Block<'a>>);

impl<'a> CenteredText<'a> {
    pub fn new(text: Text<'a>) -> Self {
        Self(text, None)
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.1 = Some(block);
        self
    }
}

impl<'a> Widget for CenteredText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let text_height: u16 = self
            .0
            .height()
            .try_into()
            .with_context(|| {
                format!(
                    "couldn't convert text height `{}` of type `usize` to `u16`",
                    self.0.height()
                )
            })
            .unwrap();
        let center = Layout::vertical([
            Constraint::Percentage((100 - text_height) / 2),
            Constraint::Percentage(text_height),
            Constraint::Percentage((100 - text_height) / 2),
        ])
        .split(self.1.inner_if_some(area))[1];

        self.1.render(area, buf);
        self.0.centered().render(center, buf);
    }
}
