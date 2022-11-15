use std::fmt;

use crate::{
    common::system::SystemPaths,
    domain::{
        entity::workspace::Workspace,
        system::{init::init_application_folders, scan::scan_workspaces_path},
    },
    infrastructure::repository::{
        create_database, error::DatabaseError, workspace_repository::WorkspaceRepository,
    },
};

use super::{error::ApplicationError, stateful_list::StatefulList, tab_state::TabsState};

#[derive(Clone, Copy)]
pub enum ApplicationStatus {
    PrepareEnvironment,
    SplashScreenReveal,
    Running,
    Quit,
}

impl fmt::Display for ApplicationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationStatus::PrepareEnvironment => write!(f, "SyncVSCode"),
            ApplicationStatus::SplashScreenReveal => write!(f, "SplashScreenReveal"),
            ApplicationStatus::Running => write!(f, "Running"),
            ApplicationStatus::Quit => write!(f, "Quit"),
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub status: ApplicationStatus,
    pub show_splash_screen: bool,
    pub workspaces: StatefulList<Workspace>,
    pub search_text: String,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, show_splash_screen: bool) -> App<'a> {
        App {
            title,
            tabs: TabsState::new(vec!["Workspaces", "Settings"]),
            status: ApplicationStatus::PrepareEnvironment,
            show_splash_screen: show_splash_screen,
            workspaces: StatefulList::with_items(vec![]),
            search_text: String::new(),
        }
    }

    pub fn on_escape_application(&mut self) {
        self.status = ApplicationStatus::Quit;
    }

    pub fn next_tab(&mut self) {
        self.tabs.next();
    }

    pub fn on_up(&mut self) {
        self.workspaces.previous();
    }

    pub fn on_down(&mut self) {
        self.workspaces.next();
    }

    pub fn enter_in_workspace(&mut self) {}

    pub fn on_key(&mut self, c: char) {
        match c {
            't' => {}
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn scan_workspaces(&mut self) -> Result<Vec<Workspace>, ApplicationError> {
        Ok(scan_workspaces_path())
    }

    fn create_database(&self, path: &str) -> Result<(), DatabaseError> {
        create_database(path)?;
        Ok(())
    }

    /// Init environment
    pub fn init_environment(&mut self) -> Result<(), ApplicationError> {
        // Default app folder create
        init_application_folders().expect("Failed to create application folders.");

        // Make sure database is always exists
        self.create_database(SystemPaths::database().as_str())
            .expect("Database cannot be created.");

        // Scan and get all workspace json file path
        let workspaces = self.scan_workspaces().expect("Scanning workspaces failed.");

        // Sync current new workspaces data to database
        let ret = WorkspaceRepository::sync_to_database(&workspaces)
            .expect("Syncing workspaces data failed.");

        self.workspaces.items = ret;
        Ok(())
    }

    pub fn state_change(&mut self, next_state: ApplicationStatus) {
        match (self.status, next_state) {
            // Starts from SyncData
            (ApplicationStatus::PrepareEnvironment, ApplicationStatus::Running) => {
                self.status = ApplicationStatus::Running
            }
            (ApplicationStatus::PrepareEnvironment, ApplicationStatus::SplashScreenReveal) => {
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
