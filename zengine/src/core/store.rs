use crate::core::entity::{Entities, Entity};

#[derive(Default, Debug)]
pub struct Store {
    entities: Entities
}

impl Store {
    pub fn build_entity(&mut self) -> Entity {
        self.entities.create_entity()
    }
}