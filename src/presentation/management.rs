use crate::application::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Line, Rectangle},
    widgets::{Block, Borders, Cell, Dataset, List, ListItem, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

/// Render vscode workspace management tab UI
fn draw_management_content_info_block<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(9), Constraint::Min(20)].as_ref())
        .split(area);
}

/// Render vscode workspace management tab UI
fn draw_management_content_search_bar<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
}

/// Render vscode workspace management tab UI
fn draw_management_content_logo_bar<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
}
