use crate::input::Input;
use sdl2::{rect::Rect, pixels::Color, render::Canvas, video::Window};
use threadpool::ThreadPool;
use std::{time::Duration, sync::mpsc::channel};

const SPEED: f64 =0.4;
const THRESHOLD: u32= 2;
pub const BASE_DEPTH: u32 = 40;


#[derive(Copy, Clone)]
pub struct Camera {
    x: f64,
    y: f64,
    screen_width:u32, 
    screen_height: u32,
    max_depth: u32,
    depth_inc: u32,
    zoom: f64,
}

struct ColoredRect {
    rect: Rect,
    color: Color,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, max_depth: u32)->Self {
        Self {
            x:0.0,
            y:0.0,
            screen_width,
            screen_height,
            max_depth,
            depth_inc: 0,
            zoom: 1.0,
        }
    }
    pub fn update_with_delta_time(&mut self, current_input: &Input, delta_time: Duration) {
        if current_input.up {
            self.y -= SPEED * delta_time.as_secs_f64() / self.zoom
        }
        if current_input.left {
            self.x -= SPEED * delta_time.as_secs_f64() / self.zoom
        }
        if current_input.down {
            self.y += SPEED * delta_time.as_secs_f64() / self.zoom
        }
        if current_input.right {
            self.x += SPEED * delta_time.as_secs_f64() / self.zoom
        }
        if current_input.zoom_in {
            self.zoom += SPEED * delta_time.as_secs_f64() * self.zoom;
            self.depth_inc+= 1;
        }
        if current_input.zoom_out {
            if self.depth_inc>0 {
                self.zoom -= SPEED * delta_time.as_secs_f64() * self.zoom;
                self.depth_inc-= 1;
            }
        }
        self.max_depth = BASE_DEPTH + (self.depth_inc/THRESHOLD);
    }
    fn screen_origin_in_world_coordinate(&self) -> (f64,f64) {
        (self.x - 1.0 / self.zoom, self.y - 1.0 / self.zoom)
    }
    fn convert_screen_coordinate_to_world_coordinate(&self, screen_x: u32, screen_y: u32) -> (f64,f64) {
        let (origin_x, origin_y)=self.screen_origin_in_world_coordinate();
        (
            2.0 * (screen_x as f64) / (self.zoom * ((self.screen_width - 1) as f64))+origin_x,
            2.0 * (screen_y as f64) / (self.zoom * ((self.screen_height - 1) as f64))+origin_y,
        )
    }
    fn get_color_of_world_coordinate(&self, world_coordinate: (f64, f64))->Color {
        let (x0, y0)=world_coordinate;

        let mut x2= 0.0;
        let mut y2= 0.0;

        let mut x= 0.0;
        let mut y= 0.0;

        let mut iteration=0;
        while x2 + y2 <= 4.0 && iteration < self.max_depth {
            y= 2.0 * x * y + y0;
            x= x2 - y2 + x0;
            x2= x * x;
            y2= y * y;
            iteration += 1;
        }
        if iteration==self.max_depth {
            Color::BLACK
        }
        else {
            Color::RGB(
                    ((3 * iteration * iteration + iteration + 4) % 255).try_into().unwrap(),
                    ((iteration + 2) % 255).try_into().unwrap(),
                    ((iteration * iteration + 5 * iteration + 20) % 255).try_into().unwrap(),
                )
        }
    }
    
    pub fn render_mandelbrot(&self, canvas: &mut Canvas<Window>) {
        self.render_mandelbrot_recursive(0, 0, self.screen_width, self.screen_height)
        .iter().for_each(|colored_rect| {
            canvas.set_draw_color(colored_rect.color);
            canvas.fill_rect(colored_rect.rect).unwrap();
        });
    }
    fn render_mandelbrot_recursive(&self, x: u32, y: u32, w: u32, h: u32) -> Vec<ColoredRect> {
        let origin = self.convert_screen_coordinate_to_world_coordinate(x, y);
        let rect_origin_color: Color = self.get_color_of_world_coordinate(origin);
        if w <= 1 && h <= 1 {
            return vec![ColoredRect {
                rect: Rect::new(x as i32, y as i32, 1, 1),
                color: rect_origin_color,
            }];
        }
        for i in x..(x + w) {
            let coordinate1 = self.convert_screen_coordinate_to_world_coordinate(i, y);
            let coordinate2 = self.convert_screen_coordinate_to_world_coordinate(i, y + h);
            let color1 = self.get_color_of_world_coordinate(coordinate1);
            let color2 = self.get_color_of_world_coordinate(coordinate2);
            if color1 != color2 || color1 != rect_origin_color {
                let mut result = Vec::new();
                if w == self.screen_width && h == self.screen_height {
                    //topmost func call
                    let n_workers = 4;
                    let pool = ThreadPool::new(n_workers);

                    let (tx, rx) = channel();

                    let tx1 = tx.clone();
                    let cam = self.clone();
                    pool.execute(move || {
                        tx1.send(cam.render_mandelbrot_recursive(x, y, w / 2 + w % 2, h / 2 + h % 2))
                            .unwrap();
                    });
                    let tx2 = tx.clone();
                    let cam = self.clone();
                    pool.execute(move || {
                        tx2.send(cam.render_mandelbrot_recursive(
                            x + w / 2 + w % 2,
                            y,
                            w / 2,
                            h / 2 + h % 2,
                        ))
                        .unwrap();
                    });
                    let cam = self.clone();
                    pool.execute(move || {
                        tx.send(cam.render_mandelbrot_recursive(
                            x,
                            y + h / 2 + h % 2,
                            w / 2 + w % 2,
                            h / 2,
                        ))
                        .unwrap();
                    });

                    result.append(&mut self.render_mandelbrot_recursive(
                        x + w / 2 + w % 2,
                        y + h / 2 + h % 2,
                        w / 2,
                        h / 2,
                    ));

                    rx.iter().for_each(|mut r| {
                        result.append(&mut r);
                    });
                } else {
                    result.append(&mut self.render_mandelbrot_recursive(
                        x,
                        y,
                        w / 2 + w % 2,
                        h / 2 + h % 2,
                    ));
                    result.append(&mut self.render_mandelbrot_recursive(
                        x + w / 2 + w % 2,
                        y,
                        w / 2,
                        h / 2 + h % 2,
                    ));
                    result.append(&mut self.render_mandelbrot_recursive(
                        x,
                        y + h / 2 + h % 2,
                        w / 2 + w % 2,
                        h / 2,
                    ));
                    result.append(&mut self.render_mandelbrot_recursive(
                        x + w / 2 + w % 2,
                        y + h / 2 + h % 2,
                        w / 2,
                        h / 2,
                    ));
                }
                return result;
            }
        }
        for j in y..(y + h) {
            let coordinate1 = self.convert_screen_coordinate_to_world_coordinate(x, j);
            let coordinate2 = self.convert_screen_coordinate_to_world_coordinate(x + w, j);
            let color1 = self.get_color_of_world_coordinate(coordinate1);
            let color2 = self.get_color_of_world_coordinate(coordinate2);
            if color1 != color2 || color1 != rect_origin_color {
                let mut result = Vec::new();
                if w == self.screen_width && h == self.screen_height {
                    //topmost func call
                    let n_workers = 4;
                    let pool = ThreadPool::new(n_workers);

                    let (tx, rx) = channel();

                    let tx1 = tx.clone();
                    let cam = self.clone();
                    pool.execute(move || {
                        tx1.send(cam.render_mandelbrot_recursive(x, y, w / 2 + w % 2, h / 2 + h % 2))
                            .unwrap();
                    });
                    let tx2 = tx.clone();
                    let cam = self.clone();
                    pool.execute(move || {
                        tx2.send(cam.render_mandelbrot_recursive(
                            x + w / 2 + w % 2,
                            y,
                            w / 2,
                            h / 2 + h % 2,
                        ))
                        .unwrap();
                    });
                    let cam = self.clone();
                    pool.execute(move || {
                        tx.send(cam.render_mandelbrot_recursive(
                            x,
                            y + h / 2 + h % 2,
                            w / 2 + w % 2,
                            h / 2,
                        ))
                        .unwrap();
                    });

                    result.append(&mut self.render_mandelbrot_recursive(
                        x + w / 2 + w % 2,
                        y + h / 2 + h % 2,
                        w / 2,
                        h / 2,
                    ));

                    rx.iter().for_each(|mut r| {
                        result.append(&mut r);
                    });
                } else {
                    result.append(&mut self.render_mandelbrot_recursive(
                        x,
                        y,
                        w / 2 + w % 2,
                        h / 2 + h % 2,
                    ));
                    result.append(&mut self.render_mandelbrot_recursive(
                        x + w / 2 + w % 2,
                        y,
                        w / 2,
                        h / 2 + h % 2,
                    ));
                    result.append(&mut self.render_mandelbrot_recursive(
                        x,
                        y + h / 2 + h % 2,
                        w / 2 + w % 2,
                        h / 2,
                    ));
                    result.append(&mut self.render_mandelbrot_recursive(
                        x + w / 2 + w % 2,
                        y + h / 2 + h % 2,
                        w / 2,
                        h / 2,
                    ));
                }
                return result;
            }
        }
        vec![ColoredRect {
            rect: Rect::new(x as i32, y as i32, w as u32, h as u32),
            color: rect_origin_color,
        }]
    }
}