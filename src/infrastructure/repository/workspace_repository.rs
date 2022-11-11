use crate::application::error::ApplicationError;
use crate::domain::entity::workspace::Workspace;
use crate::domain::repository::base::Repository;

pub struct WorkspaceRepository {}

impl Repository for WorkspaceRepository {
    type EntityType = Workspace;

    fn get_item_by_id(&self, id: &str) -> Self::EntityType {
        todo!()
    }

    fn get_all_items(&self, id: &str) -> Vec<Self::EntityType> {
        todo!()
    }
}

impl WorkspaceRepository {
    pub fn sync_to_database(curr_workspaces: &Vec<Workspace>) -> Result<(), ApplicationError> {
        Ok(())
    }
}
