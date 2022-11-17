pub trait Repository {
    type EntityType;
    fn get_item_by_id(&self, id: &str) -> Self::EntityType;
    fn get_all_items(&self) -> Vec<Self::EntityType>;
    fn insert_or_create(&self, entity: &Self::EntityType) -> Self::EntityType;
    fn delete(&self, entity: &Self::EntityType) -> bool;
    fn delete_entities(&self, entities: &Vec<Self::EntityType>) -> bool;
}
