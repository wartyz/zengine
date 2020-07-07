use crate::core::component::Component;
use crate::core::Store;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Entity(u32);

#[derive(Default, Debug)]
pub struct Entities {
    current_id: u32,
}

impl Entities {
    pub fn create_entity(&mut self) -> Entity {
        let id = self.current_id;
        self.current_id += 1;

        Entity(id)
    }
}

#[derive(Debug)]
pub struct EntityBuilder<'a> {
    entity: Entity,
    store: &'a mut Store,
    // Para saber si ha sido consumido
    is_build: bool,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity: Entity, store: &'a mut Store) -> Self {
        EntityBuilder {
            entity: entity,
            store: store,
            is_build: false,
        }
    }

    // Añade un component a la Entity
    pub fn with<C: Component>(self, component: C) -> Self {
        self.store.insert_component(&self.entity, component);
        self
    }

    // Consume la instancia para que se pueda usar de nuevo la ref mutable store: &'a mut Store,
    // y ponemos el flag is_build para saber que se ha consumido
    // por comodidad hacemos que devuelva Entity
    pub fn build(mut self) -> Entity {
        self.is_build = true;
        self.entity
    }
}

impl<'a> Drop for EntityBuilder<'a> {
    // Cuando es consumido EntityBuilder se llama a drop() que elimina el Entity de store
    fn drop(&mut self) {
        if !self.is_build {
            self.store.delete_entity(&self.entity);
        }
    }
}