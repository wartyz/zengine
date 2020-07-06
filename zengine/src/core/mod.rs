pub mod entity;
mod store;
mod scene;

// para no tener que usar scene::Scene
pub use scene::{Scene, Trans};
// para no tener que usar store::Store
pub use store::Store;