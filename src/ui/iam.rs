use crate::app::App;
use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Cell, Row, Table, TableState},
    Frame,
    layout::Rect,
};

/// Render IAM Users table
pub fn render_users(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["USER NAME", "USER ID", "ARN", "PATH", "CREATED"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header = Row::new(header_cells).height(1);

    let rows = app.filtered_iam_users.iter().map(|user| {
        let created = user.create_date.clone().unwrap_or_else(|| "-".to_string());
        // Truncate ARN for display
        let arn_display = if user.arn.len() > 40 {
            format!("{}...", &user.arn[..37])
        } else {
            user.arn.clone()
        };

        Row::new(vec![
            Cell::from(user.user_name.clone()),
            Cell::from(user.user_id.clone()),
            Cell::from(arn_display),
            Cell::from(user.path.clone()),
            Cell::from(created),
        ])
    });

    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(22),
        Constraint::Percentage(30),
        Constraint::Percentage(13),
        Constraint::Percentage(15),
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

/// Render IAM Roles table
pub fn render_roles(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["ROLE NAME", "ROLE ID", "ARN", "PATH", "CREATED"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header = Row::new(header_cells).height(1);

    let rows = app.filtered_iam_roles.iter().map(|role| {
        let created = role.create_date.clone().unwrap_or_else(|| "-".to_string());
        // Truncate ARN for display
        let arn_display = if role.arn.len() > 40 {
            format!("{}...", &role.arn[..37])
        } else {
            role.arn.clone()
        };

        Row::new(vec![
            Cell::from(role.role_name.clone()),
            Cell::from(role.role_id.clone()),
            Cell::from(arn_display),
            Cell::from(role.path.clone()),
            Cell::from(created),
        ])
    });

    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(22),
        Constraint::Percentage(28),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
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

/// Render IAM Policies table
pub fn render_policies(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["POLICY NAME", "POLICY ID", "ATTACHMENTS", "ATTACHABLE", "CREATED"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header = Row::new(header_cells).height(1);

    let rows = app.filtered_iam_policies.iter().map(|policy| {
        let created = policy.create_date.clone().unwrap_or_else(|| "-".to_string());
        let attachments = policy.attachment_count.map(|c| c.to_string()).unwrap_or_else(|| "-".to_string());
        
        let attachable_text = if policy.is_attachable { "Yes" } else { "No" };
        let attachable_style = if policy.is_attachable {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        Row::new(vec![
            Cell::from(policy.policy_name.clone()),
            Cell::from(policy.policy_id.clone()),
            Cell::from(attachments),
            Cell::from(attachable_text).style(attachable_style),
            Cell::from(created),
        ])
    });

    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(25),
        Constraint::Percentage(15),
        Constraint::Percentage(12),
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
