use ratatui::prelude::*;

pub fn center_area(area: Rect, x_size: u16, y_size: u16) -> Rect {
    let center_vert = Layout::vertical([
        Constraint::Percentage((100 - y_size) / 2),
        Constraint::Percentage(y_size),
        Constraint::Percentage((100 - y_size) / 2),
    ])
    .split(area)[1];
    Layout::horizontal([
        Constraint::Percentage((100 - x_size) / 2),
        Constraint::Percentage(x_size),
        Constraint::Percentage((100 - x_size) / 2),
    ])
    .split(center_vert)[1]
}
