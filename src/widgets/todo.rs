use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, ListState, Widget},
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
        if let Some(i) = self.selected_index() {
            return Some(&self.items[i]);
        }
        None
    }

    pub fn prev(&mut self) {
        self.widget_state.select_previous();
    }

    pub fn next(&mut self) {
        self.widget_state.select_next();
    }

    pub fn add(&mut self, item: TodoItem) {
        self.items.push(item);
    }

    pub fn delete_current(&mut self) -> Option<TodoItem> {
        if let Some(i) = self.selected_index() {
            return Some(self.items.remove(i));
        }
        None
    }

    pub fn toggle_current_status(&mut self) {
        if let Some(i) = self.selected_index() {
            let item = &mut self.items[i];
            item.toggle_status();
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn selected_index(&self) -> Option<usize> {
        match self.widget_state.selected() {
            Some(i) if !self.is_empty() => Some(i),
            _ => None,
        }
    }

    fn render_empty_todo_list(&self, area: Rect, buf: &mut Buffer) {
        CenteredText::new(Text::from("Todo list empty").cyan())
            .block(Block::default().white().on_black())
            .render(area, buf);
    }

    fn render_todo_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items = self.items.iter().enumerate().map(|(i, item)| {
            let prefix = match item.status() {
                TodoItemStatus::InProgress => {
                    Span::styled("in progress", Style::default().light_blue())
                }
                TodoItemStatus::Complete => {
                    Span::styled("complete   ", Style::default().light_green())
                }
            };

            let item_line = Line::from(vec![
                prefix,
                Span::from(" | "),
                Span::from(format!("(#{}) ", i.to_string())).light_cyan(),
                Span::from(item.title()),
            ]);
            ListItem::new(item_line).style(TodoList::alternate_color(i))
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
        if self.is_empty() {
            self.render_empty_todo_list(area, buf);
        } else {
            self.render_todo_list(area, buf);
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
