use std::sync::RwLock;
use super::entity as entity;

pub struct DataHolder {
    pub data: RwLock<Vec<entity::Object>>,
}
