mod app;
mod aws;
mod config;
mod event;
mod resource;
mod ui;

use anyhow::Result;
use app::App;
use config::Config;
use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;
use ui::splash::{SplashState, render as render_splash};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Show splash screen and initialize
    let result = initialize_with_splash(&mut terminal).await;

    match result {
        Ok(Some(mut app)) => {
            // Run the main app
            let run_result = run_app(&mut terminal, &mut app).await;

            // Restore terminal
            cleanup_terminal(&mut terminal)?;

            if let Err(err) = run_result {
                eprintln!("Error: {err:?}");
            }
        }
        Ok(None) => {
            // User aborted during initialization
            cleanup_terminal(&mut terminal)?;
        }
        Err(err) => {
            // Restore terminal before showing error
            cleanup_terminal(&mut terminal)?;
            eprintln!("Initialization error: {err:?}");
        }
    }

    Ok(())
}

fn cleanup_terminal<B: Backend + std::io::Write>(terminal: &mut Terminal<B>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn initialize_with_splash<B: Backend>(terminal: &mut Terminal<B>) -> Result<Option<App>> {
    let mut splash = SplashState::new();

    // Render initial splash
    terminal.draw(|f| render_splash(f, &splash))?;

    // Check for abort
    if check_abort()? {
        return Ok(None);
    }

    // Step 1: Load configuration (env vars override saved config)
    let config = Config::load();
    let profile = config.effective_profile();
    let region = config.effective_region();
    
    splash.set_message(&format!("Loading AWS config [profile: {}]", profile));
    terminal.draw(|f| render_splash(f, &splash))?;
    splash.complete_step();

    if check_abort()? {
        return Ok(None);
    }

    // Step 2: Initialize all AWS clients
    splash.set_message(&format!("Connecting to AWS services [{}]", region));
    terminal.draw(|f| render_splash(f, &splash))?;

    let (clients, actual_region) = aws::client::AwsClients::new(&profile, &region).await?;
    splash.complete_step();

    if check_abort()? {
        return Ok(None);
    }

    // Step 3: Load profiles
    splash.set_message("Reading ~/.aws/config");
    terminal.draw(|f| render_splash(f, &splash))?;

    let available_profiles = aws::profiles::list_profiles().unwrap_or_else(|_| vec!["default".to_string()]);
    let available_regions = aws::profiles::list_regions();
    splash.complete_step();

    if check_abort()? {
        return Ok(None);
    }

    // Step 4: Fetch EC2 instances using new dynamic system
    splash.set_message(&format!("Fetching instances from {}", actual_region));
    terminal.draw(|f| render_splash(f, &splash))?;

    let (instances, initial_error) = {
        // Use the new JSON-driven resource system
        match resource::fetch_resources("ec2-instances", &clients, &[]).await {
            Ok(items) => (items, None),
            Err(e) => {
                let error_msg = aws::client::format_aws_error(&e);
                (Vec::new(), Some(error_msg))
            }
        }
    };

    splash.complete_step();
    splash.set_message("Ready!");
    terminal.draw(|f| render_splash(f, &splash))?;

    // Small delay to show completion
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Create the app with config
    let mut app = App::from_initialized(
        clients,
        profile,
        actual_region,
        available_profiles,
        available_regions,
        instances,
        config,
    );

    // Set initial error if any
    if let Some(err) = initial_error {
        app.error_message = Some(err);
    }

    Ok(Some(app))
}

fn check_abort() -> Result<bool> {
    if poll(Duration::from_millis(50))? {
        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        // Handle user input
        if event::handle_events(app).await? {
            return Ok(());
        }
        
        // Auto-refresh every 5 seconds (only in Normal mode)
        if app.needs_refresh() {
            let _ = app.refresh_current().await;
        }
    }
}
