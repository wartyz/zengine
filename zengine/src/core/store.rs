use crate::core::entity::{Entities, Entity, EntityBuilder};
use crate::core::component::{Components, Component};

#[derive(Default, Debug)]
pub struct Store {
    entities: Entities,
    components: Components,
}

impl Store {
    pub fn build_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self.entities.create_entity(), self)
    }

    pub fn delete_entity(&mut self, entity: &Entity) {
        self.components.delete_entity(entity);
    }

    pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.components.insert(entity, component);
    }
}