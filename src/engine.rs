use crate::camera;

use super::{camera::Camera, input::Input};
use sdl2::{
    self, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump,
};
use std::time::{Duration, Instant};

pub struct Engine {
    current_input: Input,
    canvas: Canvas<Window>,
    event_listener: EventPump,
    camera: Camera,
}

impl Engine {
    pub fn new_square_screen(screen_size: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window("Mandelbrot", screen_size, screen_size)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_listener = sdl.event_pump().unwrap();
        Self {
            current_input: Input::new(),
            canvas,
            event_listener,
            camera: Camera::new(screen_size, screen_size, camera::BASE_DEPTH),
        }
    }
    pub fn new_rect_screen(screen_width: u32, screen_height: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window("Mandelbrot", screen_width, screen_height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_listener = sdl.event_pump().unwrap();
        Self {
            current_input: Input::new(),
            canvas,
            event_listener,
            camera: Camera::new(screen_width, screen_height, camera::BASE_DEPTH),
        }
    }
    pub fn engine_loop(&mut self) {
        let mut time_elapsed: Duration = Duration::ZERO;
        let mut now: Instant;
        while !self.current_input.exit {
            now = Instant::now();
            self.process_input();
            self.camera
                .update_with_delta_time(&self.current_input, time_elapsed);
            self.render();
            time_elapsed = Instant::now() - now;
            let remaining = ((1_000_000_u32 / 60) as i64) - time_elapsed.as_millis() as i64;
            if remaining > 0 {
                std::thread::sleep(Duration::from_nanos(remaining as u64));
            }
        }
    }
    fn process_input(&mut self) {
        for event in self.event_listener.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.current_input.exit = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.current_input.left = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.current_input.right = true,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => self.current_input.up = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.current_input.down = true,
                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => self.current_input.zoom_in = true,
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => self.current_input.zoom_out = true,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => self.current_input.left = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => self.current_input.right = false,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => self.current_input.up = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => self.current_input.down = false,
                Event::KeyUp {
                    keycode: Some(Keycode::O),
                    ..
                } => self.current_input.zoom_in = false,
                Event::KeyUp {
                    keycode: Some(Keycode::P),
                    ..
                } => self.current_input.zoom_out = false,
                _ => {}
            }
        }
    }
    fn render(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.camera.render_mandelbrot(&mut self.canvas);
        self.canvas.present();
    }
}
