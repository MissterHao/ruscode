use std::fmt;
use tui::widgets::ListState;

use crate::domain::system::scan::scan_workspaces_path;

#[derive(Clone, Copy)]
pub enum ApplicationStatus {
    SyncVSCode,
    SplashScreenReveal,
    Running,
    Quit,
}

impl fmt::Display for ApplicationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationStatus::SyncVSCode => write!(f, "SyncVSCode"),
            ApplicationStatus::SplashScreenReveal => write!(f, "SplashScreenReveal"),
            ApplicationStatus::Running => write!(f, "Running"),
            ApplicationStatus::Quit => write!(f, "Quit"),
        }
    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
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

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub status: ApplicationStatus,
    pub show_splash_screen: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, show_splash_screen: bool) -> App<'a> {
        App {
            title,
            tabs: TabsState::new(vec!["Workspaces", "Settings"]),
            status: ApplicationStatus::SyncVSCode,
            show_splash_screen: show_splash_screen,
        }
    }

    pub fn on_escape_application(&mut self) {
        self.status = ApplicationStatus::Quit;
    }

    pub fn next_tab(&mut self) {
        self.tabs.next();
    }

    pub fn on_up(&mut self) {
        // self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        // self.tasks.next();
    }

    pub fn enter_in_workspace(&mut self) {}

    pub fn on_key(&mut self, c: char) {
        match c {
            't' => {}
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn scan_workspaces(&mut self) {
        scan_workspaces_path();
    }

    pub fn state_change(&mut self, next_state: ApplicationStatus) {
        match (self.status, next_state) {
            // Starts from SyncData
            (ApplicationStatus::SyncVSCode, ApplicationStatus::Running) => {
                self.status = ApplicationStatus::Running
            }
            (ApplicationStatus::SyncVSCode, ApplicationStatus::SplashScreenReveal) => {
                self.status = ApplicationStatus::SplashScreenReveal
            }

            // Starts from Splash Screen
            (ApplicationStatus::SplashScreenReveal, ApplicationStatus::Running) => {
                self.status = ApplicationStatus::Running
            }

            // Starts from Running
            (ApplicationStatus::Running, ApplicationStatus::Quit) => {
                self.status = ApplicationStatus::Quit
            }
            // () => self.status = ApplicationStatus::Quit,
            _ => panic!("Cannot transit from {} to {}", &self.status, &next_state),
        };
    }
}
