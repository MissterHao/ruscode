
pub trait Repository {
    type EntityType;
    fn get_item_by_id(&self, id: &str) -> Self::EntityType;
    fn get_all_items(&self, id: &str) -> Vec<Self::EntityType>;
    fn insert_or_create(&self, entity: Self::EntityType) -> Self::EntityType;
}


