use crate::domain::repository::base::Repository;
use crate::domain::entity::workspace::Workspace;


struct WorkspaceRepository {
    
}

impl Repository for WorkspaceRepository {
    type EntityType = Workspace;

    fn get_item_by_id(&self, id: &str) -> Self::EntityType {
        todo!()
    }

    fn get_all_items(&self, id: &str) -> Vec<Self::EntityType> {
        todo!()
    }

}
