use rusqlite::params;
use std::collections::HashSet;
use std::result;

use crate::application::error::ApplicationError;
use crate::common::system::SystemPaths;
use crate::domain::entity::workspace::Workspace;
use crate::domain::repository::base::Repository;

use super::get_db_connection;

pub struct WorkspaceRepository {}

impl Repository for WorkspaceRepository {
    type EntityType = Workspace;

    fn get_item_by_id(&self, _id: &str) -> Self::EntityType {
        todo!()
    }

    fn get_all_items(&self) -> Vec<Self::EntityType> {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        let mut stmt = db_connection
            .prepare(r#"SELECT * FROM workspaces"#)
            .expect("Failed t   o select all workspaces.");

        let workspace_iter = stmt
            .query_map([], |row| Ok(Workspace::from_dbrow(row)))
            .expect("Failed to transform database row to entity.");

        workspace_iter
            .map(|x| x.unwrap())
            .collect::<Vec<Workspace>>()
    }

    fn insert_or_create(&self, entity: &Self::EntityType) -> Self::EntityType {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        let mut stmt = db_connection
            .prepare(r#"INSERT INTO workspaces (path) VALUES (?1)"#)
            .expect("Failed t   o select all workspaces.");

        stmt.execute(params![entity.path])
            .expect("Cannot insert to db");

        Self::EntityType::from(entity.path.as_str())
    }

    fn delete(&self, entity: &Self::EntityType) -> bool {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        let mut stmt = db_connection
            .prepare(r#"DELETE FROM workspaces where"#)
            .expect("Failed to select all workspaces.");

        true
    }

    fn delete_entities(&self, entities: &Vec<Self::EntityType>) -> bool {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        let mut stmt = db_connection
            .prepare(r#"DELETE FROM workspaces where path = ?1"#)
            .expect("Failed to select all workspaces.");

        for entity in entities {
            stmt.execute([entity.path.clone()]);
        }

        true
    }
}

impl WorkspaceRepository {
    pub fn filter(sql: String) -> Result<Vec<Workspace>, ApplicationError> {
        let db_connection = get_db_connection(SystemPaths::database().as_str())
            .expect("Cannot get database connection.");

        println!("{sql}");
        let mut stmt = db_connection
            .prepare(&sql)
            .expect("Failed to select all workspaces.");

        let workspace_iter = stmt
            .query_map([], |row| Ok(Workspace::from_dbrow(row)))
            .expect("Failed to transform database row to entity.");

        Ok(workspace_iter
            .map(|x| x.unwrap())
            .collect::<Vec<Workspace>>())
    }

    pub fn sync_to_database(
        curr_workspaces: &Vec<Workspace>,
    ) -> Result<Vec<Workspace>, ApplicationError> {
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
        let insert_list = in_folder
            .difference(&in_db)
            .map(|x| x.clone())
            .collect::<Vec<Workspace>>();
        // In folder but not in db anymore
        let delete_list = in_db
            .difference(&in_folder)
            .map(|x| x.clone())
            .collect::<Vec<Workspace>>();

        // Create Workspace repository
        let workspace_repo = WorkspaceRepository {};
        for w in insert_list {
            workspace_repo.insert_or_create(&w);
        }

        // Delete Workspace repository
        let workspace_repo = WorkspaceRepository {};
        workspace_repo.delete_entities(&delete_list);

        Ok(stmt
            .query_map([], |row| Ok(Workspace::from_dbrow(row)))
            .unwrap()
            .map(|x| x.unwrap())
            .collect::<Vec<Workspace>>())
    }
}
