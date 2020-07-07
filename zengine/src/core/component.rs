use downcast_rs::Downcast;
use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::collections::HashMap;
use crate::core::entity::Entity;

pub trait Component: Any + Debug {}

#[derive(Debug, Default)]
pub struct Components {
    // TypeId
    storages: HashMap<TypeId, Box<dyn AnySet>>,
}

impl Components {
    pub fn insert<C: Component>(&mut self, entity: &Entity, component: C) {
        // Creamos un TypeID para cada Component
        let type_id = TypeId::of::<C>();

        match self.storages.get_mut(&type_id) {
            Some(storage) => {
                // Ya existe
                storage
                    .downcast_mut::<Set<C>>()
                    .expect("downcast set error")
                    .insert(entity.clone
                    (), component);
            }
            None => {
                // No existe, hay que crearlo
                let mut storage = Set::<C>::default();
                storage.insert(entity.clone(), component);

                self.storages.insert(type_id, Box::new(storage));
            }
        }
    }

    pub fn delete_entity(&mut self, entity: &Entity) {
        for s in self.storages.iter_mut() {
            s.1.remove(&entity);
        }
    }
}


pub type Set<C> = HashMap<Entity, C>;

impl<C: Component> AnySet for Set<C> {
    fn remove(&mut self, entity: &Entity) {
        self.remove(entity);
    }
}

pub trait AnySet: Downcast + Debug {
    fn remove(&mut self, entity: &Entity) {}
}
downcast_rs::impl_downcast!(AnySet);