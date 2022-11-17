//! Management Tab UI Render functions.
//!
//! Use these function to render workspace management tab UI components.
//! There are two main blocks:
//! - Control Block
//! - Content Information Block
//!
//! ## Control Block
//! User can send keyboard event to control the app:
//! - search by workspace name
//! - search by self defined tags
//! - keyboard up & down to traversal the list of workspaces which match user's search pattern
//!
//! ## Content Information Block:
//! Display deatil information of selected vscode workspace.

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::application::app::App;

use super::{
    style::RuscodeStyle,
    text::{DETAIL_MODE_HELP_TEXT, SEARCH_MODE_HELP_TEXT},
};

/// Display detail information of selected vscode workspace
///  
pub fn draw_management_content_info_block<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Split area in to chunks
    let chunks = Layout::default()
        .constraints([Constraint::Min(30)].as_ref())
        .split(area);

    let p = Paragraph::new("Workspace detail 🔍")
        .style(match app.control_mode {
            crate::application::app::ApplicationControlMode::SearchMode => {
                RuscodeStyle::unfocus_mode()
            }
            crate::application::app::ApplicationControlMode::DetailMode => {
                RuscodeStyle::focus_mode()
            }
        })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Rounded),
        );
    f.render_widget(p, chunks[0]);
}

/// Render vscode workspace management tab UI
pub fn draw_management_control_block<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Split area in to chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    draw_management_control_upper_bar(f, app, chunks[0]);
    draw_management_control_workspace_list(f, app, chunks[1]);
}

/// Render vscode workspace management tab UI
fn draw_management_control_upper_bar<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Split area in to chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Define and render "Search bar" block
    let p = Paragraph::new(app.search_text.as_ref())
        .style(match app.control_mode {
            crate::application::app::ApplicationControlMode::SearchMode => {
                RuscodeStyle::default_focus_mode()
            }
            crate::application::app::ApplicationControlMode::DetailMode => {
                RuscodeStyle::unfocus_mode()
            }
        })
        .block(Block::default().borders(Borders::ALL).title(" Search "));
    f.render_widget(p, chunks[0]);

    // Define and render "help text" block
    let help_text_paragraph = Paragraph::new(match app.control_mode {
        crate::application::app::ApplicationControlMode::SearchMode => SEARCH_MODE_HELP_TEXT,
        crate::application::app::ApplicationControlMode::DetailMode => DETAIL_MODE_HELP_TEXT,
    })
    .block(Block::default().borders(Borders::ALL))
    .style(RuscodeStyle::success());
    f.render_widget(help_text_paragraph, chunks[1]);

    // If the application is currently in Search mode ( which means search text is not empty )
    // then, use UnicodeWidth to control position of cursor
    if app.search_text.len() > 0 {
        use unicode_width::UnicodeWidthStr;
        f.set_cursor(
            // Put cursor past the end of the input text
            chunks[0].x + app.search_text.width() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[0].y + 1,
        )
    }
}

/// Render vscode workspace management tab UI
fn draw_management_control_workspace_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Split area in to chunks
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    let items = List::new(
        app.filtered_workspaces()
            .iter()
            .map(|x| {
                let lines = vec![
                    Spans::from(Span::styled(
                        x.title.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )),
                    Spans::from(Span::styled(
                        x.decode_path.clone(),
                        Style::default().add_modifier(Modifier::DIM),
                    )),
                ];
                ListItem::new(lines).style(match app.control_mode {
                    crate::application::app::ApplicationControlMode::SearchMode => {
                        RuscodeStyle::default_font()
                    }
                    crate::application::app::ApplicationControlMode::DetailMode => {
                        RuscodeStyle::unfocus_mode()
                    }
                })
            })
            .collect::<Vec<ListItem>>(),
    )
    .style(match app.control_mode {
        crate::application::app::ApplicationControlMode::SearchMode => {
            RuscodeStyle::default_focus_mode()
        }
        crate::application::app::ApplicationControlMode::DetailMode => RuscodeStyle::unfocus_mode(),
    })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Workspace List 📜 "),
    )
    .highlight_style(
        Style::default()
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol("📌 ");
    f.render_stateful_widget(items, chunks[0], &mut app.workspaces.state);
}
