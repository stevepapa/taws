use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let Some(instance) = app.selected_instance() else {
        return;
    };

    let mut lines: Vec<Line> = vec![
        create_detail_line_owned("Instance ID", &instance.instance_id),
        create_detail_line_owned("Name", &instance.name),
        create_detail_line_owned("State", &instance.state),
        create_detail_line_owned("Type", &instance.instance_type),
        create_detail_line_owned("AZ", &instance.availability_zone),
        Line::from(""),
        create_section_header("Network"),
        create_detail_line_owned("VPC ID", instance.vpc_id.as_deref().unwrap_or("-")),
        create_detail_line_owned("Subnet ID", instance.subnet_id.as_deref().unwrap_or("-")),
        create_detail_line_owned("Private IP", instance.private_ip.as_deref().unwrap_or("-")),
        create_detail_line_owned("Public IP", instance.public_ip.as_deref().unwrap_or("-")),
        Line::from(""),
        create_section_header("Instance Details"),
        create_detail_line_owned("Platform", instance.platform.as_deref().unwrap_or("Linux/UNIX")),
        create_detail_line_owned("Architecture", instance.architecture.as_deref().unwrap_or("-")),
        create_detail_line_owned("Key Name", instance.key_name.as_deref().unwrap_or("-")),
        create_detail_line_owned("Launch Time", instance.launch_time.as_deref().unwrap_or("-")),
        create_detail_line_owned("Monitoring", instance.monitoring_state.as_deref().unwrap_or("-")),
        Line::from(""),
        create_section_header("Storage"),
        create_detail_line_owned("Root Device", instance.root_device_name.as_deref().unwrap_or("-")),
        create_detail_line_owned("Root Type", instance.root_device_type.as_deref().unwrap_or("-")),
    ];

    // Add block devices
    for (i, device) in instance.block_devices.iter().enumerate() {
        if i == 0 {
            lines.push(Line::from(""));
            lines.push(create_section_header("Block Devices"));
        }
        let device_info = format!("{} ({})", device.volume_id, device.status);
        lines.push(create_detail_line_owned(&device.device_name, &device_info));
    }

    // Add security groups
    if !instance.security_groups.is_empty() {
        lines.push(Line::from(""));
        lines.push(create_section_header("Security Groups"));
        for sg in &instance.security_groups {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(sg.clone(), Style::default().fg(Color::White)),
            ]));
        }
    }

    // Add IAM profile
    if let Some(iam) = &instance.iam_instance_profile {
        lines.push(Line::from(""));
        lines.push(create_section_header("IAM"));
        lines.push(create_detail_line_owned("Profile", iam));
    }

    // Add tags
    if !instance.tags.is_empty() {
        lines.push(Line::from(""));
        lines.push(create_section_header("Tags"));
        for (key, value) in &instance.tags {
            lines.push(create_detail_line_owned(key, value));
        }
    }

    // Apply scroll offset
    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(app.describe_scroll)
        .collect();

    let block = Block::default()
        .title(" Details ")
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .borders(Borders::LEFT);

    let paragraph = Paragraph::new(visible_lines)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn create_detail_line_owned(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{:>14}: ", label),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(value.to_string(), Style::default().fg(Color::White)),
    ])
}

fn create_section_header(title: &str) -> Line<'static> {
    Line::from(vec![Span::styled(
        format!("── {} ", title),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )])
}
