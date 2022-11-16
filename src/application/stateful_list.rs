use std::fmt::Debug;

use tui::widgets::ListState;
pub struct StatefulList<T>
where
    T: Debug,
{
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T>
where
    T: Debug,
{
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn has_selected_item(&self) -> bool {
        match self.state.selected() {
            Some(_) => true,
            None => false,
        }
    }

    pub fn unselected(&mut self) {
        self.state.select(None);
    }

    pub fn next(&mut self) {
        if self.items.len() <= 0 {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.len() <= 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
