use anyhow::Context;
use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
};

use super::CenteredText;

pub enum TodoItemStatus {
    InProgress,
    Complete,
}

pub struct TodoItem {
    title: String,
    description: String,
    status: TodoItemStatus,
}

#[derive(Default)]
pub struct TodoList {
    items: Vec<TodoItem>,
    widget_state: ListState,
}

impl TodoItem {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            status: TodoItemStatus::InProgress,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &TodoItemStatus {
        &self.status
    }

    pub fn toggle_status(&mut self) {
        match self.status {
            TodoItemStatus::InProgress => self.status = TodoItemStatus::Complete,
            TodoItemStatus::Complete => self.status = TodoItemStatus::InProgress,
        }
    }
}

impl TodoList {
    pub fn selected(&self) -> Option<&TodoItem> {
        if let Some(i) = self.widget_state.selected() {
            return Some(&self.items[i]);
        }
        None
    }

    pub fn select_prev(&mut self) {
        self.widget_state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.widget_state.select_next();
    }

    pub fn add_todo_item(&mut self, item: TodoItem) {
        self.items.push(item);
    }

    pub fn delete_selected(&mut self) -> Option<TodoItem> {
        if let Some(i) = self.widget_state.selected() {
            return Some(self.items.remove(i));
        }
        None
    }

    pub fn toggle_current_item_status(&mut self) {
        if let Some(i) = self.widget_state.selected() {
            let item = &mut self.items[i];
            item.toggle_status();
        }
    }

    fn empty_todo_list_view(&self, area: Rect, buf: &mut Buffer) {
        CenteredText::new("Hello world".into()).render(area, buf);
    }

    fn todo_list_view(&mut self, area: Rect, buf: &mut Buffer) {
        let items = self.items.iter().enumerate().map(|(i, item)| {
            let prefix = match item.status() {
                TodoItemStatus::InProgress => {
                    Span::styled("in progress", Style::default().light_blue())
                }
                TodoItemStatus::Complete => {
                    Span::styled("complete   ", Style::default().light_green())
                }
            };
            let line = Line::from(vec![
                prefix,
                Span::from(" | "),
                Span::from(format!("(#{}) ", i.to_string())).light_cyan(),
                Span::from(item.title()),
            ]);
            ListItem::new(line).style(TodoList::alternate_color(i))
        });
        let list = List::new(items)
            .highlight_symbol("> ")
            .highlight_style(Style::default().black().on_white().bold())
            .block(Block::default().white().on_black());
        StatefulWidget::render(list, area, buf, &mut self.widget_state);
    }

    fn alternate_color(n: usize) -> Style {
        if n % 2 == 0 {
            Style::default().on_dark_gray()
        } else {
            Style::default().on_black()
        }
    }
}

impl Widget for &mut TodoList {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if self.items.is_empty() {
            self.empty_todo_list_view(area, buf);
        } else {
            self.todo_list_view(area, buf);
        }
    }
}

impl std::fmt::Display for TodoItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoItemStatus::Complete => write!(f, "complete"),
            TodoItemStatus::InProgress => write!(f, "in progress"),
        }
    }
}