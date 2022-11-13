use crate::application::error::ApplicationError;
use crate::common::system::SystemPaths;
use crate::domain::entity::workspace::Workspace;
use crate::domain::repository::base::Repository;

use super::get_db_connection;

use std::collections::HashSet;

pub struct WorkspaceRepository {}

impl Repository for WorkspaceRepository {
    type EntityType = Workspace;

    fn get_item_by_id(&self, id: &str) -> Self::EntityType {
        todo!()
    }

    fn get_all_items(&self, id: &str) -> Vec<Self::EntityType> {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        let mut stmt = db_connection
            .prepare(r#"SELECT * FROM workspaces"#)
            .expect("Failed t   o select all workspaces.");

        let workspace_iter = stmt
            .query_map([], |row| Ok(Workspace::from_dbrow(row)))
            .expect("Failed to transform database row to entity.");
        vec![]
    }

    fn insert_or_create(&self, entity: Self::EntityType) -> Self::EntityType {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        let mut stmt = db_connection
            .prepare(r#"SELECT * FROM workspaces"#)
            .expect("Failed t   o select all workspaces.");

        let workspace_iter = stmt
            .query_map([], |row| Ok(Workspace::from_dbrow(row)))
            .expect("Failed to transform database row to entity.");

        for w in workspace_iter {
            println!("{:?}", w.unwrap());
        }

        Self::EntityType::new()
    }
}

impl WorkspaceRepository {
    pub fn sync_to_database(curr_workspaces: &Vec<Workspace>) -> Result<(), ApplicationError> {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        let mut stmt = db_connection
            .prepare(r#"SELECT path FROM workspaces"#)
            .expect("Failed to select all workspaces.");

        let workspace_iter = stmt
            .query_map([], |row| Ok(Workspace::from_dbrow(row)))
            .expect("Failed to transform database row to entity.");

        let in_db_vec = workspace_iter
            .map(|x| x.unwrap())
            .collect::<Vec<Workspace>>();

        let in_db: HashSet<Workspace> = HashSet::from_iter(in_db_vec.iter().cloned());
        let in_folder: HashSet<Workspace> = HashSet::from_iter(curr_workspaces.iter().cloned());

        // In db but not in folder anymore
        // println!("{:?}", in_db.difference(&in_folder));

        // In folder but not in db anymore
        // println!("{:?}", in_folder.difference(&in_db));

        Ok(())
    }
}
