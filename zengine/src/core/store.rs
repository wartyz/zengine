use crate::core::entity::{Entities, Entity, EntityBuilder};
use crate::core::component::{Components, Component, Set};

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

    pub fn get_components<C: Component>(&self) -> Option<&Set<C>> {
        self.components.get::<C>()
    }

    pub fn get_components_mut<C: Component>(&mut self) -> Option<&mut Set<C>> {
        self.components.get_mut::<C>()
    }

    pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.components.insert(entity, component);
    }
}