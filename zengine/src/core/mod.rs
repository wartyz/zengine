pub mod entity;
mod store;
mod scene;
mod component;

// para no tener que usar scene::Scene
pub use scene::{Scene, Trans};
// para no tener que usar store::Store
pub use store::Store;
// para no tener que usar component::Component
pub use component::Component;