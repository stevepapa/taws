use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let Some(instance) = app.selected_instance() else {
        return;
    };

    // Create centered popup area (90% of screen)
    let area = centered_rect(90, 90, f.area());

    // Clear the area behind the popup
    f.render_widget(Clear, area);

    // Serialize instance to pretty JSON
    let json = serde_json::to_string_pretty(instance).unwrap_or_else(|_| "{}".to_string());

    let lines: Vec<&str> = json.lines().collect();
    let visible_lines: String = lines
        .iter()
        .skip(app.describe_scroll)
        .take(area.height as usize - 2) // Account for borders
        .cloned()
        .collect::<Vec<&str>>()
        .join("\n");

    let title = format!(
        " {} - JSON ({}/{}) ",
        instance.instance_id,
        app.describe_scroll + 1,
        lines.len()
    );

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(visible_lines)
        .block(block)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

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
