use crate::app::{App, ConfirmAction};
use crate::resource::extract_json_value;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let Some(item) = app.selected_item() else {
        return;
    };

    let Some(action) = &app.confirm_action else {
        return;
    };

    let area = centered_rect(50, 20, f.area());

    f.render_widget(Clear, area);

    let (title, message) = match action {
        ConfirmAction::Terminate => {
            let name = if let Some(resource) = app.current_resource() {
                extract_json_value(item, &resource.name_field)
            } else {
                "-".to_string()
            };
            let id = if let Some(resource) = app.current_resource() {
                extract_json_value(item, &resource.id_field)
            } else {
                "-".to_string()
            };
            
            (
                " Terminate Instance ",
                format!(
                    "Are you sure you want to terminate {}?\n\nInstance: {}\n\nThis action cannot be undone!",
                    name, id
                ),
            )
        }
        ConfirmAction::Custom(action_name) => {
            let name = if let Some(resource) = app.current_resource() {
                extract_json_value(item, &resource.name_field)
            } else {
                "-".to_string()
            };
            (
                " Confirm Action ",
                format!("Are you sure you want to {} on {}?", action_name, name),
            )
        }
    };

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[y]",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Yes  "),
            Span::styled(
                "[n]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" No"),
        ]),
    ];

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
