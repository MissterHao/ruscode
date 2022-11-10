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
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Display detail information of selected vscode workspace
///  
pub fn draw_management_content_info_block<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Min(30)].as_ref())
        .split(area);

    let p = Paragraph::new("Content info")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Rounded),
        );
    f.render_widget(p, chunks[0]);
}

/// Render vscode workspace management tab UI
pub fn draw_management_control_block<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    draw_management_control_upper_bar(f, chunks[0]);
    draw_management_control_workspace_list(f, chunks[1]);
}

/// Render vscode workspace management tab UI
fn draw_management_control_upper_bar<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let p = Paragraph::new("").block(Block::default().borders(Borders::ALL).title(" Search "));
    let p2 = Paragraph::new("Logo").block(Block::default().borders(Borders::ALL));
    f.render_widget(p, chunks[0]);
    f.render_widget(p2, chunks[1]);
}

/// Render vscode workspace management tab UI
fn draw_management_control_workspace_list<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    let p = Paragraph::new("Workspace List").block(Block::default().borders(Borders::ALL));
    f.render_widget(p, chunks[0]);
}
