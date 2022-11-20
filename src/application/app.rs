use std::{fmt, vec};

use crate::{
    common::system::SystemPaths,
    domain::{
        entity::workspace::Workspace,
        searching::parse::SearchingStrategy,
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

#[derive(Clone, Copy)]
pub enum ApplicationControlMode {
    SearchMode,
    DetailMode,
}

impl Default for ApplicationControlMode {
    fn default() -> Self {
        Self::SearchMode
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub status: ApplicationStatus,
    pub control_mode: ApplicationControlMode,
    pub show_splash_screen: bool,
    pub workspaces: StatefulList<Workspace>,
    pub search_text: String,
    workspaces_source: Vec<Workspace>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, show_splash_screen: bool) -> App<'a> {
        App {
            title,
            tabs: TabsState::new(vec!["Workspaces", "Settings"]),
            status: ApplicationStatus::PrepareEnvironment,
            control_mode: ApplicationControlMode::default(),
            show_splash_screen: show_splash_screen,
            workspaces: StatefulList::with_items(vec![]),
            search_text: String::new(),
            workspaces_source: vec![],
        }
    }

    pub fn select_workspace(&self) -> Option<&Workspace> {
        self.workspaces.selected_item()
    }

    pub fn on_escape_application(&mut self) {
        self.status = ApplicationStatus::Quit;
    }

    pub fn next_tab(&mut self) {
        self.tabs.next();
    }

    pub fn enter_detail_mode(&mut self) {
        if !self.workspaces.has_selected_item() {
            return;
        };

        match self.control_mode {
            ApplicationControlMode::SearchMode => {
                self.control_mode = ApplicationControlMode::DetailMode;
            }
            _ => {}
        }
    }

    pub fn enter_search_mode(&mut self) {
        match self.control_mode {
            ApplicationControlMode::DetailMode => {
                self.control_mode = ApplicationControlMode::SearchMode;
            }
            _ => {}
        }
    }

    pub fn on_up_list(&mut self) {
        match self.control_mode {
            ApplicationControlMode::SearchMode => {
                self.workspaces.previous();
            }
            ApplicationControlMode::DetailMode => {}
        }
    }

    pub fn on_down_list(&mut self) {
        match self.control_mode {
            ApplicationControlMode::SearchMode => {
                self.workspaces.next();
            }
            ApplicationControlMode::DetailMode => {}
        }
    }

    pub fn on_enter(&mut self) {
        match self.control_mode {
            ApplicationControlMode::SearchMode => self.open_workspace(),
            ApplicationControlMode::DetailMode => self.enter_new_tag(),
        }
    }

    pub fn on_backspace(&mut self) {
        match self.control_mode {
            ApplicationControlMode::SearchMode => {
                self.search_text.pop();
                self.workspaces
                    .change_item_source(self.filtered_workspaces());
            }
            ApplicationControlMode::DetailMode => {
                todo!()
            }
        }
    }

    pub fn filtered_workspaces(&self) -> Vec<Workspace> {
        let strategy: SearchingStrategy = self.search_text.clone().into();

        match strategy.searching_type {
            crate::domain::searching::parse::SearchingStrategyType::All => {
                self.workspaces_source.clone()
            }
            crate::domain::searching::parse::SearchingStrategyType::Tags => todo!(),
            crate::domain::searching::parse::SearchingStrategyType::PlainText => self
                .workspaces_source
                .clone()
                .iter()
                .filter(|x| x.path.contains(&self.search_text))
                .map(|x| x.clone())
                .collect(),
            crate::domain::searching::parse::SearchingStrategyType::PlainTextMixTags => todo!(),
        }
    }

    /// Open workspace by vscode
    fn open_workspace(&mut self) {}

    /// Enter new tag for selected Workspace
    fn enter_new_tag(&mut self) {}

    //
    pub fn on_key(&mut self, c: char) {
        match self.control_mode {
            ApplicationControlMode::SearchMode => self.on_input_search_text(c),
            ApplicationControlMode::DetailMode => todo!(),
        }
    }

    fn on_input_search_text(&mut self, c: char) {
        self.search_text.push(c);
    }

    /// Scan all workspace record generated by vscode
    pub fn scan_workspaces(&mut self) -> Result<Vec<Workspace>, ApplicationError> {
        Ok(scan_workspaces_path())
    }

    /// Create database if not exist
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

        self.workspaces_source = ret;

        Ok(())
    }

    /// Manage all ApplicationStatus transition
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
