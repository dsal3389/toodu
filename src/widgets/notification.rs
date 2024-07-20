use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph, Widget},
};
use std::{
    iter::zip,
    time::{Duration, Instant},
};

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

    // creates a new notification on the stack
    // with the defined duration in the notification
    pub fn push_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    // returns a boolean value indicating if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    // remove notification from stack which duration time exceeded
    fn remove_timedout_notifications(&mut self) {
        self.notifications = self
            .notifications
            .iter()
            .filter(|n| n.should_be_displayed())
            .cloned()
            .collect();
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

    // boolean value indicating if the notification
    // duration was exceeded
    pub fn should_be_displayed(&self) -> bool {
        let now = Instant::now();
        match now.checked_duration_since(self.initilized_time) {
            Some(d) => d < self.duration,
            None => false,
        }
    }

    // returns the area with minimum required width
    fn limit_area_width(&self, area: Rect) -> Rect {
        Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length((self.content.len() + 4) as u16),
        ])
        .split(area)[1]
    }
}

impl Widget for &mut NotificationStack {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.remove_timedout_notifications();
        if self.is_empty() {
            return;
        }

        let max_notifications = area.height / 3;
        let mut constraints = Vec::new();

        for _ in 0..max_notifications {
            constraints.push(Constraint::Ratio(1, max_notifications as u32));
        }

        let areas = Layout::vertical(constraints).split(area);
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
        let area = self.limit_area_width(area);
        let p = Paragraph::new(self.content.clone()).block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Thick)
                .title(self.title.clone())
                .border_style(match self.level {
                    NotificationLevel::Info => Style::default().light_blue(),
                    NotificationLevel::Warn => Style::default().light_yellow(),
                    NotificationLevel::Error => Style::default().light_red(),
                })
                .white()
                .on_black(),
        );

        Widget::render(Clear, area, buf);
        p.render(area, buf);
    }
}
