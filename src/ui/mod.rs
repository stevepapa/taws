mod command_box;
mod dialog;
mod header;
mod help;
mod profiles;
mod regions;
pub mod splash;

use crate::app::{App, Mode};
use crate::resource::{extract_json_value, ColumnDef, get_color_for_value};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Header (multi-line)
            Constraint::Min(1),    // Main content (table or describe)
            Constraint::Length(1), // Footer/crumb
        ])
        .split(f.area());

    // Header - multi-line with context info
    header::render(f, app, chunks[0]);

    // Main content - depends on mode and view
    match app.mode {
        Mode::Profiles => {
            profiles::render(f, app, chunks[1]);
        }
        Mode::Regions => {
            regions::render(f, app, chunks[1]);
        }
        Mode::Describe => {
            render_describe_view(f, app, chunks[1]);
        }
        _ => {
            render_main_content(f, app, chunks[1]);
        }
    }

    // Footer/crumb
    render_crumb(f, app, chunks[2]);

    // Overlays
    match app.mode {
        Mode::Help => {
            help::render(f, app);
        }
        Mode::Confirm => {
            dialog::render(f, app);
        }
        Mode::Command => {
            command_box::render(f, app);
        }
        _ => {}
    }
}

fn render_main_content(f: &mut Frame, app: &App, area: Rect) {
    // If filter is active or has text, show filter input above table
    let show_filter = app.filter_active || !app.filter_text.is_empty();
    
    if show_filter {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(area);
        
        render_filter_bar(f, app, chunks[0]);
        render_dynamic_table(f, app, chunks[1]);
    } else {
        render_dynamic_table(f, app, area);
    }
}

fn render_filter_bar(f: &mut Frame, app: &App, area: Rect) {
    let cursor_style = if app.filter_active {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    
    let filter_display = if app.filter_active {
        format!("/{}_", app.filter_text)
    } else {
        format!("/{}", app.filter_text)
    };
    
    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled(filter_display, cursor_style),
    ]));
    f.render_widget(paragraph, area);
}

/// Render dynamic table based on current resource definition
fn render_dynamic_table(f: &mut Frame, app: &App, area: Rect) {
    let Some(resource) = app.current_resource() else {
        let msg = Paragraph::new("Unknown resource")
            .style(Style::default().fg(Color::Red));
        f.render_widget(msg, area);
        return;
    };

    // Build title with count and region info
    let title = {
        let count = app.filtered_items.len();
        let total = app.items.len();
        let is_global = resource.is_global;
        
        if is_global {
            if app.filter_text.is_empty() {
                format!(" {}[{}] ", resource.display_name, count)
            } else {
                format!(" {}[{}/{}] ", resource.display_name, count, total)
            }
        } else if app.filter_text.is_empty() {
            format!(" {}({})[{}] ", resource.display_name, app.region, count)
        } else {
            format!(" {}({})[{}/{}] ", resource.display_name, app.region, count, total)
        }
    };

    // Create the bordered box with centered title
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Build header from column definitions with left padding
    let header_cells = resource.columns.iter().map(|col| {
        Cell::from(format!(" {}", col.header)).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    });
    let header = Row::new(header_cells).height(1);

    // Build rows from filtered items with left padding
    let rows = app.filtered_items.iter().map(|item| {
        let cells = resource.columns.iter().map(|col| {
            let value = extract_json_value(item, &col.json_path);
            let style = get_cell_style(&value, col);
            Cell::from(format!(" {}", truncate_string(&value, 38))).style(style)
        });
        Row::new(cells)
    });

    // Build column widths
    let widths: Vec<Constraint> = resource
        .columns
        .iter()
        .map(|col| Constraint::Percentage(col.width))
        .collect();

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

    f.render_stateful_widget(table, inner_area, &mut state);
}

/// Get cell style based on value and column definition
fn get_cell_style(value: &str, col: &ColumnDef) -> Style {
    if let Some(ref color_map_name) = col.color_map {
        if let Some([r, g, b]) = get_color_for_value(color_map_name, value) {
            return Style::default().fg(Color::Rgb(r, g, b));
        }
    }
    Style::default()
}

/// Truncate string for display
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

fn render_describe_view(f: &mut Frame, app: &App, area: Rect) {
    let json = app.selected_item_json().unwrap_or_else(|| "No item selected".to_string());
    let lines: Vec<Line> = json.lines().map(|l| Line::from(l.to_string())).collect();
    let total_lines = lines.len();
    
    let max_scroll = total_lines.saturating_sub(area.height as usize);
    let scroll = app.describe_scroll.min(max_scroll);
    
    let title = if let Some(resource) = app.current_resource() {
        format!(" {} Details ", resource.display_name)
    } else {
        " Details ".to_string()
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            title,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);
    
    let paragraph = Paragraph::new(lines.clone())
        .style(Style::default().fg(Color::White))
        .scroll((scroll as u16, 0));
    
    f.render_widget(paragraph, inner_area);
    
    if total_lines > inner_area.height as usize {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        let mut scrollbar_state = ScrollbarState::new(total_lines)
            .position(scroll);
        f.render_stateful_widget(scrollbar, inner_area, &mut scrollbar_state);
    }
}

fn render_crumb(f: &mut Frame, app: &App, area: Rect) {
    // Build breadcrumb from navigation
    let breadcrumb = app.get_breadcrumb();
    let crumb_display = breadcrumb.join(" > ");
    
    // Build sub-resource shortcuts hint
    let shortcuts_hint = if let Some(resource) = app.current_resource() {
        if !resource.sub_resources.is_empty() && app.mode == Mode::Normal {
            let hints: Vec<String> = resource
                .sub_resources
                .iter()
                .map(|s| format!("{}:{}", s.shortcut, s.display_name))
                .collect();
            format!(" | {}", hints.join(" "))
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let status_text = if let Some(err) = &app.error_message {
        format!("Error: {}", err)
    } else if app.loading {
        "Loading...".to_string()
    } else if app.mode == Mode::Describe {
        "j/k: scroll | q/d/Esc: back".to_string()
    } else if app.filter_active {
        "Type to filter | Enter: apply | Esc: clear".to_string()
    } else {
        shortcuts_hint
    };

    let style = if app.error_message.is_some() {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else if app.loading {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let crumb = Line::from(vec![
        Span::styled(
            format!("<{}>", crumb_display),
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" "),
        Span::styled(status_text, style),
    ]);

    let paragraph = Paragraph::new(crumb);
    f.render_widget(paragraph, area);
}
