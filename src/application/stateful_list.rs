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

    pub fn selected_item(&self) -> Option<&T> {
        match self.state.selected() {
            Some(idx) => self.items.get(idx),
            None => None,
        }
    }

    pub fn change_item_source(&mut self, items: Vec<T>) {
        self.items = items;
        match self.state.selected() {
            Some(curr) => {
                if curr > self.items.len() {
                    self.state.select(Some(0));
                }
            }
            None => self.state.select(None),
        }
    }

    pub fn has_selected_item(&self) -> bool {
        self.state.selected().is_some()
    }

    #[allow(dead_code)]
    pub fn unselected(&mut self) {
        self.state.select(None);
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
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
        if self.items.is_empty() {
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
