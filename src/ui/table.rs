use crate::app::App;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Cell, Row, Table, TableState},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Column headers with sort indicator (NAME↑ like k9s)
    let header_cells = ["NAME↑", "INSTANCE ID", "STATE", "TYPE", "AZ", "PRIVATE IP"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header = Row::new(header_cells).height(1);

    let rows = app.filtered_instances.iter().map(|instance| {
        let state_style = match instance.state.as_str() {
            "running" => Style::default().fg(Color::Green),
            "stopped" => Style::default().fg(Color::Red),
            "pending" | "stopping" | "shutting-down" => Style::default().fg(Color::Yellow),
            "terminated" => Style::default().fg(Color::DarkGray),
            _ => Style::default(),
        };

        // Show only last char of AZ (e.g., "a" instead of "us-east-1a")
        let az_short = instance
            .availability_zone
            .chars()
            .last()
            .map(|c| c.to_string())
            .unwrap_or_else(|| instance.availability_zone.clone());

        Row::new(vec![
            Cell::from(instance.name.clone()),
            Cell::from(instance.instance_id.clone()),
            Cell::from(instance.state.clone()).style(state_style),
            Cell::from(instance.instance_type.clone()),
            Cell::from(az_short),
            Cell::from(instance.private_ip.clone().unwrap_or_else(|| "-".to_string())),
        ])
    });

    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(20),
        Constraint::Percentage(12),
        Constraint::Percentage(15),
        Constraint::Percentage(8),
        Constraint::Percentage(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = TableState::default();
    state.select(Some(app.selected));

    f.render_stateful_widget(table, area, &mut state);
}
