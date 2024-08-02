use ratatui::{crossterm::event::KeyCode, prelude::*};

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

pub trait View {
    fn view_event_key(&mut self, key: KeyCode);

    fn render_view(&mut self, area: Rect, buf: &mut Buffer);
}

mod list;
mod new_task;

pub use list::ListView;
pub use new_task::NewTaskView;
