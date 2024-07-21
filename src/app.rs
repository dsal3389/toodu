use anyhow::Context;
use ratatui::{
    crossterm::event::{poll, read, Event, KeyCode, KeyEventKind},
    layout::Layout,
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
    Terminal,
};
use std::time::Duration;

use crate::widgets::{
    CenteredText, Notification, NotificationLevel, NotificationStack, TodoItem, TodoList,
};

#[derive(Default, PartialEq)]
pub enum ApplicationState {
    #[default]
    Running,
    Exiting,
}

#[derive(Default)]
pub enum ApplicationInputState {
    #[default]
    Normal,
    Writing,
}

#[derive(Default)]
pub enum ApplicationView {
    #[default]
    TodoListView,
    TodoItemAdd,
}

#[derive(Default)]
pub struct Application {
    running_state: ApplicationState,
    input_state: ApplicationInputState,
    view: ApplicationView,
    notifications: NotificationStack,
    todo_list: TodoList,
}

impl Application {
    pub fn load_data(&mut self) {
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
        self.todo_list
            .add_todo_item(TodoItem::new("hello world".into(), "description 1".into()));
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
        while self.running_state != ApplicationState::Exiting {
            terminal
                .draw(|f| f.render_widget(&mut *self, f.size()))
                .context("couldn't draw new frame to terminal screen")?;
            self.handle_input_events()?;
        }
        Ok(())
    }

    fn handle_input_events(&mut self) -> anyhow::Result<()> {
        if poll(Duration::from_millis(250))? {
            if let Event::Key(key_event) = read().context("couldn't read input key event")? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event.code);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyCode) {
        match self.view {
            ApplicationView::TodoListView => match key {
                KeyCode::Char('q') | KeyCode::Esc => self.running_state = ApplicationState::Exiting,
                KeyCode::Char('j') | KeyCode::Down => self.todo_list.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.todo_list.select_prev(),
                KeyCode::Char('d') | KeyCode::Delete => match self.todo_list.delete_selected() {
                    Some(item) => self.notifications.push_notification(Notification::new(
                        " deleted item ".into(),
                        format!(
                            "deleted item `{}` from todo list with status {}",
                            item.title(),
                            item.status()
                        ),
                        Duration::from_secs(7),
                        NotificationLevel::Warn,
                    )),
                    None => {}
                },
                KeyCode::Char('n') => self.view = ApplicationView::TodoItemAdd,
                KeyCode::Enter | KeyCode::Tab => self.todo_list.toggle_current_item_status(),
                _ => {}
            },
            // on the todo item add view, we need to respect the
            // current application input state, because the user might be typing
            // into an input box
            ApplicationView::TodoItemAdd => match self.input_state {
                ApplicationInputState::Normal => match key {
                    KeyCode::Char('q') | KeyCode::Esc => self.view = ApplicationView::TodoListView,
                    _ => {}
                },
                ApplicationInputState::Writing => match key {
                    KeyCode::Esc => self.input_state = ApplicationInputState::Normal,
                    _ => {}
                },
            },
        }
    }

    fn render_list_view(&mut self, area: Rect, buf: &mut Buffer) {
        let [list_area, content_area, controls_area] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Length(1),
        ])
        .areas(area);

        self.todo_list.render(list_area, buf);
        self.render_todo_item_content(content_area, buf);
        self.render_controls(controls_area, buf);
    }

    fn render_item_add_view(&mut self, area: Rect, buf: &mut Buffer) {}

    fn render_todo_item_content(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::TOP).white().on_black();
        match self.todo_list.selected() {
            Some(item) => {
                let inner_block = Block::bordered()
                    .padding(Padding::horizontal(1))
                    .border_style(Style::default().light_blue())
                    .on_black();
                let inner_area = Layout::default()
                    .vertical_margin(1)
                    .horizontal_margin(5)
                    .constraints([Constraint::Min(0)])
                    .split(block.inner(area))[0];

                block.render(area, buf);
                Paragraph::new(item.description().clone())
                    .block(inner_block)
                    .render(inner_area, buf);
            }
            None => {
                CenteredText::new(
                    Text::from("scroll on some tasks to view thier content here").cyan(),
                )
                .block(block)
                .render(area, buf);
            }
        }
    }

    fn render_controls(&mut self, area: Rect, buf: &mut Buffer) {
        #[macro_export]
        macro_rules! key_spans {
            ($( $key:expr, $description:expr ),*) => {
                vec![
                    $(
                        Span::from(format!(" {} - {} ", $key, $description)),
                        Span::from("|").black(),
                    )*
                ]
            }
        }

        match self.view {
            ApplicationView::TodoListView => {
                Line::from(key_spans!(
                    "k/j",
                    "UP/DN",
                    "TAB/Enter",
                    "toggle task status",
                    "n",
                    "new task",
                    "d/DEL",
                    "delete task",
                    "q/esc",
                    "quit"
                ))
                .black()
                .on_white()
                .bold()
                .render(area, buf);
            }
            ApplicationView::TodoItemAdd => {}
        };
    }
}

impl Widget for &mut Application {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.view {
            ApplicationView::TodoListView => self.render_list_view(area, buf),
            ApplicationView::TodoItemAdd => self.render_item_add_view(area, buf),
        }

        if !self.notifications.is_empty() {
            self.notifications.render(area, buf);
        }
    }
}
