extern crate zengine;

// Como hemos puesto en lib.rs, pub use engine::Engine; podemos usar esto:
use zengine::Engine;
use zengine::core::{Scene, Store, Trans, Component};


fn main() {
    Engine::default().run(Game {
        execution_number: 10,
    });
}

// Creamos dos Components
#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct Test {
    data: u32,
}

impl Component for Position {}

impl Component for Test {}

pub struct Game {
    execution_number: u32,
}

impl Scene for Game {
    fn on_start(&mut self, store: &mut Store) {
        println!("start de Game scene");

        // Construye Entity con los dos Component
        // Como tenemos una referencia store: &'a mut Store en EntityBuilder
        // y sólo puede existir una referncia mutable a la vez,
        // para poder construir otra Entity deberiamos encerrar esto entre {} para que al salir
        // del alcance se pueda crear otra referencia mutable a store, como esto es incómodo
        // usamos el método build() al final, que consume la instancia,
        // (la recibe pero no la devuelve), luego por comodidad hacemos que build devuelva Entity
        let e = store
            .build_entity()
            .with(Position { x: 43.0, y: 3.5 })
            .with(Test { data: 5 })
            .build();
        println!("Entity {:?}", e);
        {
            let e2 = store
                .build_entity()
                .with(Test { data: 8 })
                .build();
            println!("Entity2 {:?}", e2);
        }


        println!("Store {:?}", store);
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