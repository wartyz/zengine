extern crate zengine;

// Como hemos puesto en lib.rs, pub use engine::Engine; podemos usar esto:
use zengine::Engine;
use zengine::core::{Scene, Store, Trans};


fn main() {
    Engine::default().run(Game {
        execution_number: 10,
    });
}

pub struct Game {
    execution_number: u32,
}

impl Scene for Game {
    fn on_start(&mut self, store: &mut Store) {
        println!("start de Game scene");
        let e = store.build_entity();

        println!("Entity {:?}", e);
    }


    fn on_stop(&mut self, store: &mut Store) {
        println!("stop de Game scene");
    }

    fn update(&mut self, store: &mut Store) -> Trans {
        match self.execution_number {
            0 => Trans::Quit,
            _ => {
                println!("Store = {:?}", store);
                self.execution_number -= 1;
                Trans::None
            }
        }
    }
}