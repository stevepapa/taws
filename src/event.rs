use crate::app::{App, ConfirmAction, Mode};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub async fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            return handle_key_event(app, key).await;
        }
    }
    Ok(false)
}

async fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<bool> {
    match app.mode {
        Mode::Normal => handle_normal_mode(app, key).await,
        Mode::Command => handle_command_mode(app, key).await,
        Mode::Help => handle_help_mode(app, key),
        Mode::Describe => handle_describe_mode(app, key),
        Mode::Confirm => handle_confirm_mode(app, key).await,
        Mode::Profiles => handle_profiles_mode(app, key).await,
        Mode::Regions => handle_regions_mode(app, key).await,
    }
}

// Region shortcuts matching the header display
const REGION_SHORTCUTS: &[&str] = &[
    "us-east-1",
    "us-west-2",
    "eu-west-1",
    "eu-central-1",
    "ap-northeast-1",
    "ap-southeast-1",
];

async fn handle_normal_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    // If filter is active, handle filter input
    if app.filter_active {
        return handle_filter_input(app, key).await;
    }

    match key.code {
        // Quit with Ctrl+C
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(true),

        // Region shortcuts (0-5)
        KeyCode::Char('0') => {
            if let Some(region) = REGION_SHORTCUTS.first() {
                app.switch_region(region).await?;
                app.refresh_current().await?;
            }
        }
        KeyCode::Char('1') => {
            if let Some(region) = REGION_SHORTCUTS.get(1) {
                app.switch_region(region).await?;
                app.refresh_current().await?;
            }
        }
        KeyCode::Char('2') => {
            if let Some(region) = REGION_SHORTCUTS.get(2) {
                app.switch_region(region).await?;
                app.refresh_current().await?;
            }
        }
        KeyCode::Char('3') => {
            if let Some(region) = REGION_SHORTCUTS.get(3) {
                app.switch_region(region).await?;
                app.refresh_current().await?;
            }
        }
        KeyCode::Char('4') => {
            if let Some(region) = REGION_SHORTCUTS.get(4) {
                app.switch_region(region).await?;
                app.refresh_current().await?;
            }
        }
        KeyCode::Char('5') => {
            if let Some(region) = REGION_SHORTCUTS.get(5) {
                app.switch_region(region).await?;
                app.refresh_current().await?;
            }
        }

        // Navigation - vim style
        KeyCode::Char('j') | KeyCode::Down => app.next(),
        KeyCode::Char('k') | KeyCode::Up => app.previous(),
        KeyCode::Home => app.go_to_top(),
        KeyCode::Char('G') | KeyCode::End => app.go_to_bottom(),

        // Page navigation
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // ctrl+d = page down (or terminate in EC2 view)
            if app.current_resource_key == "ec2-instances" {
                app.enter_confirm_mode(ConfirmAction::Terminate);
            } else {
                app.page_down(10);
            }
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.page_up(10);
        }
        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.page_down(10);
        }
        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.page_up(10);
        }

        // Describe mode (d or Enter)
        KeyCode::Char('d') => app.enter_describe_mode(),
        KeyCode::Enter => app.enter_describe_mode(),

        // Filter toggle
        KeyCode::Char('/') => {
            app.toggle_filter();
        }

        // Mode switches
        KeyCode::Char(':') => app.enter_command_mode(),
        KeyCode::Char('?') => app.enter_help_mode(),

        // Backspace goes back in navigation
        KeyCode::Backspace => {
            if app.parent_context.is_some() {
                app.navigate_back().await?;
            }
        }

        // Escape clears filter if present
        KeyCode::Esc => {
            if !app.filter_text.is_empty() {
                app.clear_filter();
            } else if app.parent_context.is_some() {
                app.navigate_back().await?;
            }
        }

        // Dynamic shortcuts: sub-resources and EC2 actions
        _ => {
            if let KeyCode::Char(c) = key.code {
                let mut handled = false;
                
                // Check if it's a sub-resource shortcut for current resource
                if let Some(resource) = app.current_resource() {
                    for sub in &resource.sub_resources {
                        if sub.shortcut == c.to_string() && app.selected_item().is_some() {
                            app.navigate_to_sub_resource(&sub.resource_key).await?;
                            handled = true;
                            break;
                        }
                    }
                }
                
                 // EC2-specific actions (only if nothing else matched)
                // Note: EC2 has 'v' for volumes, so 's' and 'S' are free for start/stop
                if !handled && app.current_resource_key == "ec2-instances" {
                    match c {
                        's' => {
                            app.start_selected_instance().await?;
                        }
                        'S' => {
                            app.stop_selected_instance().await?;
                        }
                        _ => {}
                    }
                }

                // Handle 'gg' for go_to_top
                if c == 'g' {
                    if let Some((last_key, last_time)) = app.last_key_press {
                        if last_key == KeyCode::Char('g') && last_time.elapsed() < Duration::from_millis(250) {
                            app.go_to_top();
                            app.last_key_press = None;
                            handled = true;
                        }
                    }
                }
                if !handled && c == 'g' {
                    app.last_key_press = Some((KeyCode::Char('g'), std::time::Instant::now()));
                } else {
                    app.last_key_press = None;
                }
            }
        }
    }
    Ok(false)
}

async fn handle_filter_input(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.clear_filter();
        }
        KeyCode::Enter => {
            app.filter_active = false;
        }
        KeyCode::Backspace => {
            app.filter_text.pop();
            app.apply_filter();
        }
        KeyCode::Char(c) => {
            app.filter_text.push(c);
            app.apply_filter();
        }
        _ => {}
    }
    Ok(false)
}

async fn handle_command_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.command_text.clear();
            app.exit_mode();
        }
        KeyCode::Enter => {
            let should_quit = app.execute_command().await?;
            if should_quit {
                return Ok(true);
            }
            if app.mode == Mode::Command {
                app.exit_mode();
            }
        }
        KeyCode::Tab | KeyCode::Right => {
            app.apply_suggestion();
        }
        KeyCode::Down => {
            app.next_suggestion();
        }
        KeyCode::Up => {
            app.prev_suggestion();
        }
        KeyCode::Backspace => {
            app.command_text.pop();
            app.update_command_suggestions();
        }
        KeyCode::Char(c) => {
            app.command_text.push(c);
            app.update_command_suggestions();
        }
        _ => {}
    }
    Ok(false)
}

fn handle_help_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
            app.exit_mode();
        }
        _ => {}
    }
    Ok(false)
}

fn handle_describe_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.exit_mode();
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.describe_scroll = app.describe_scroll.saturating_add(10);
        }
        KeyCode::Char('d') => {
            app.exit_mode();
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.describe_scroll = app.describe_scroll.saturating_sub(10);
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.describe_scroll = app.describe_scroll.saturating_add(1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.describe_scroll = app.describe_scroll.saturating_sub(1);
        }
        KeyCode::Char('g') | KeyCode::Home => {
            app.describe_scroll = 0;
        }
        KeyCode::Char('G') | KeyCode::End => {
            app.describe_scroll = usize::MAX / 2;
        }
        _ => {}
    }
    Ok(false)
}

async fn handle_confirm_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if let Some(ConfirmAction::Terminate) = &app.confirm_action {
                app.terminate_selected_instance().await?;
            }
            app.exit_mode();
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.exit_mode();
        }
        _ => {}
    }
    Ok(false)
}

async fn handle_profiles_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.exit_mode();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.previous();
        }
        KeyCode::Char('g') | KeyCode::Home => {
            app.go_to_top();
        }
        KeyCode::Char('G') | KeyCode::End => {
            app.go_to_bottom();
        }
        KeyCode::Enter => {
            app.select_profile().await?;
        }
        _ => {}
    }
    Ok(false)
}

async fn handle_regions_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.exit_mode();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.previous();
        }
        KeyCode::Char('g') | KeyCode::Home => {
            app.go_to_top();
        }
        KeyCode::Char('G') | KeyCode::End => {
            app.go_to_bottom();
        }
        KeyCode::Enter => {
            app.select_region().await?;
        }
        _ => {}
    }
    Ok(false)
}
