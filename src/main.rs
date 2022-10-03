mod input;
mod engine;
mod camera; 

use engine::Engine;


fn main() {
    let mut engine=Engine::new_rect_screen(1280, 720);
    engine.engine_loop();
}
