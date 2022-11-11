use crate::application::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use super::management::{draw_management_content_info_block, draw_management_control_block};

const FIGLET_ASCII_LOGO: &'static str = r#"



                              _      
 _ __ _   _ ___  ___ ___   __| | ___
| '__| | | / __|/ __/ _ \ / _` |/ _ \
| |  | |_| \__ \ (_| (_) | (_| |  __/
|_|   \__,_|___/\___\___/ \__,_|\___|




"#;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.status {
        crate::application::app::ApplicationStatus::Quit => {}
        crate::application::app::ApplicationStatus::Running => draw_application(f, app),
        crate::application::app::ApplicationStatus::SplashScreenReveal
        | crate::application::app::ApplicationStatus::PrepareEnvironment => {
            draw_splash_screen(f, app)
        }
    }
}

/// Render Default Application UI
///
/// # Arguments
/// * `f` - Franme
/// * `app` - App struct
fn draw_application<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    // Main blocks
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    // Tabs' title
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::DarkGray))))
        .collect();

    // Create tab and configure style
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::LightBlue))
        .select(app.tabs.index);

    // Render tabs to terminal
    f.render_widget(tabs, chunks[0]);

    // Render tab content to terminal
    match app.tabs.index {
        0 => draw_management_tab(f, app, chunks[1]),
        1 => draw_settings_tab(f, app, chunks[1]),
        _ => {}
    };
}

/// Render Splash Screen for ruscode
///
/// # Arguments
/// * `f` - Franme
/// * `app` - App struct
fn draw_splash_screen<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    // Main blocks
    let chunks = Layout::default()
        .constraints([Constraint::Min(20)].as_ref())
        .split(f.size());

    let p = Paragraph::new(FIGLET_ASCII_LOGO).alignment(Alignment::Center);

    // Render tabs to terminal
    f.render_widget(p, chunks[0]);
}

/// Render vscode workspace management tab UI
///
/// # Arguments
/// * `f` - Franme
/// * `app` - App struct
/// * `area` - area of frame
fn draw_management_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Min(40)].as_ref())
        .split(area);

    draw_management_control_block(f, chunks[0]);
    draw_management_content_info_block(f, chunks[1]);
}

/// Render ruscode setting tab UI
///
/// # Arguments
/// * `f` - Franme
/// * `app` - App struct
/// * `area` - area of frame
fn draw_settings_tab<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);
}
