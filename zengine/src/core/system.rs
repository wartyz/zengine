use crate::core::store::Store;
use std::fmt::Debug;
use std::any::Any;

// El trait Any creo que es algo de static
pub trait System: Any + Debug {
    fn init(&mut self, store: &mut Store) {}

    fn run(&mut self, store: &Store) {}

    fn dispose(&mut self, store: &mut Store) {}
}