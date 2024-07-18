use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph, Widget},
};
use std::{
    iter::zip,
    time::{Duration, Instant},
};

const MAX_NOTIFICATIONS: u32 = 7;

#[derive(Default, Clone)]
pub enum NotificationLevel {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Clone)]
pub struct Notification {
    title: String,
    content: String,
    duration: Duration,
    initilized_time: Instant,
    level: NotificationLevel,
}

#[derive(Default)]
pub struct NotificationStack {
    notifications: Vec<Notification>,
}

impl NotificationStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cleanup(&mut self) {
        self.notifications = self
            .notifications
            .iter()
            .filter(|n| n.should_be_displayed())
            .cloned()
            .collect();
    }

    pub fn push_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }
}

impl Notification {
    pub fn new(
        title: String,
        content: String,
        duration: Duration,
        level: NotificationLevel,
    ) -> Self {
        Self {
            title,
            content,
            duration,
            initilized_time: Instant::now(),
            level,
        }
    }

    pub fn should_be_displayed(&self) -> bool {
        let now = Instant::now();
        match now.checked_duration_since(self.initilized_time) {
            Some(d) => d < self.duration,
            None => false,
        }
    }
}

impl Widget for &NotificationStack {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if self.is_empty() {
            return;
        }

        let areas =
            Layout::vertical([Constraint::Ratio(1, MAX_NOTIFICATIONS); MAX_NOTIFICATIONS as usize])
                .vertical_margin(1)
                .horizontal_margin(3)
                .split(area);
        for (area, notification) in zip(areas.iter(), &self.notifications[0..]) {
            notification.render(*area, buf);
        }
    }
}

impl Widget for &Notification {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let box_border_style = match self.level {
            NotificationLevel::Info => Style::default().light_blue(),
            NotificationLevel::Warn => Style::default().light_yellow(),
            NotificationLevel::Error => Style::default().light_red(),
        };

        Widget::render(Clear, area, buf);
        Paragraph::new(self.content.clone())
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(self.title.clone())
                    .border_style(box_border_style)
                    .white()
                    .on_black(),
            )
            .render(area, buf);
    }
}
