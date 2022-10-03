mod input;
mod engine;
mod camera; 

use engine::Engine;


fn main() {
    let mut engine=Engine::new_square_screen(1000);
    engine.engine_loop();
}
