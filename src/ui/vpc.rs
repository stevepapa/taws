use crate::app::App;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Cell, Row, Table, TableState},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["NAME", "VPC ID", "STATE", "CIDR", "DEFAULT", "TENANCY"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header = Row::new(header_cells).height(1);

    let rows = app.filtered_vpcs.iter().map(|vpc| {
        let state_style = match vpc.state.as_str() {
            "available" => Style::default().fg(Color::Green),
            "pending" => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        };

        let default_text = if vpc.is_default { "Yes" } else { "No" };
        let default_style = if vpc.is_default {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        Row::new(vec![
            Cell::from(vpc.name.clone()),
            Cell::from(vpc.vpc_id.clone()),
            Cell::from(vpc.state.clone()).style(state_style),
            Cell::from(vpc.cidr_block.clone()),
            Cell::from(default_text).style(default_style),
            Cell::from(vpc.instance_tenancy.clone()),
        ])
    });

    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(22),
        Constraint::Percentage(12),
        Constraint::Percentage(18),
        Constraint::Percentage(10),
        Constraint::Percentage(18),
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
