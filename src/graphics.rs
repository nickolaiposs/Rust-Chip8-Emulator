use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cpu::DISPLAY_SIZE;

const SCALE: usize = 15;

pub struct Graphics {
    canvas: Canvas<Window>
}

impl Graphics {
    pub fn new(sdl_context: &Sdl, display_size: (u32, u32)) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", display_size.0, display_size.1)
        .position_centered()
        .build()
        .expect("Could not initialize display");
    
        let canvas = window.into_canvas().build().unwrap();

        Self {
            canvas
        }
    }

    pub fn render(&mut self, pixel_data: &[[u8; DISPLAY_SIZE.1]; DISPLAY_SIZE.0]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        for (x, row) in pixel_data.iter().enumerate() {
            for (y, &col) in row.iter().enumerate() {
                let x_coord = (x * SCALE) as i32;
                let y_coord = (y * SCALE) as i32;
                let rect = Rect::new(x_coord, y_coord, SCALE as u32, SCALE as u32);

                self.canvas.set_draw_color(get_color(col));
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        
        self.canvas.present();
    }

    pub fn set_title(&mut self, title: &String) {
        self.canvas.window_mut().set_title(title).unwrap();
    }
}

fn get_color(color_val: u8) -> Color {
    if color_val == 0 {
        Color::RGB(15, 15, 15)
    } else {
        Color::RGB(200, 200, 200)
    }
}