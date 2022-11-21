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

use std::{error::Error, vec};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    application::app::App,
    domain::{entity::workspace, system::folder_observer::last_modified},
};

use super::{
    error::UIError,
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

    let workspace_detail: Vec<Spans>;
    match app.select_workspace() {
        Some(selected_workspace) => {
            workspace_detail = get_workspace_detail_text(selected_workspace).unwrap();
        }
        None => {
            workspace_detail = vec![Spans::from(vec![Span::raw(
                "More detail information, please use arrow key to select workspace.",
            )])];
        }
    }

    let p = Paragraph::new(workspace_detail)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Workspace detail 🔍")
                .borders(Borders::ALL)
                .style(match app.control_mode {
                    crate::application::app::ApplicationControlMode::SearchMode => {
                        RuscodeStyle::unfocus_mode()
                    }
                    crate::application::app::ApplicationControlMode::DetailMode => {
                        RuscodeStyle::focus_mode()
                    }
                })
                .border_type(tui::widgets::BorderType::Rounded),
        );
    f.render_widget(p, chunks[0]);
}

fn get_workspace_detail_text(
    selected_workspace: &workspace::Workspace,
) -> Result<Vec<Spans>, UIError> {
    let last_modified_span_text = match selected_workspace.location_type {
        workspace::WorkspaceLocation::NotRecognize => Spans::from(vec![Span::raw("")]),
        workspace::WorkspaceLocation::Local => match last_modified(selected_workspace) {
            Ok(val) => Spans::from(vec![
                Span::raw("Last modified time: "),
                Span::styled(val, Style::default().fg(Color::Yellow)),
            ]),
            Err(_) => Spans::from(vec![Span::styled(
                "Couldn't access selected workspace's directory path at local",
                Style::default().fg(Color::Red),
            )]),
        },
        workspace::WorkspaceLocation::Remote => Spans::from(vec![Span::raw("")]),
    };

    Ok(vec![
        Spans::from(vec![
            Span::raw("Workspace Name: "),
            Span::styled(
                selected_workspace.title.clone(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Workspace Path: "),
            match selected_workspace.location_type {
                workspace::WorkspaceLocation::NotRecognize => Span::styled(
                    "Can't recognize this workspace type.",
                    Style::default().fg(Color::Red),
                ),
                workspace::WorkspaceLocation::Local => Span::styled(
                    selected_workspace.strip_decode_path(),
                    Style::default().fg(Color::Yellow),
                ),
                workspace::WorkspaceLocation::Remote => Span::styled(
                    "This is a remote workspace",
                    Style::default().fg(Color::Yellow),
                ),
            },
        ]),
        last_modified_span_text,
    ])
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

    app.workspaces.change_item_source(app.filtered_workspaces());

    let items = List::new(
        app.workspaces
            .items
            .iter()
            .map(|x| {
                let lines = vec![
                    Spans::from(Span::styled(
                        x.title.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )),
                    Spans::from(Span::styled(
                        x.strip_decode_path().clone(),
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
