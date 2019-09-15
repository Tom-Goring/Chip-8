use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;

use crate::CHIP8_WIDTH;
use crate::CHIP8_HEIGHT;

const PIXEL_SIZE: u32 = 20;
const SCREEN_WIDTH: u32 =(CHIP8_WIDTH as u32) * PIXEL_SIZE;
const SCREEN_HEIGHT: u32 =(CHIP8_HEIGHT as u32) * PIXEL_SIZE;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Chip8 Emulator",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        DisplayDriver { canvas: canvas }
    }

    pub fn draw(&mut self, pixel_buffer: &[bool; CHIP8_WIDTH * CHIP8_HEIGHT]) { // TODO: pass in pixel buffer to draw here

        for row in 0..CHIP8_HEIGHT {
            for column in 0..CHIP8_WIDTH {
                let current_pixel_index = (row * CHIP8_WIDTH) + column;
                if pixel_buffer[current_pixel_index] {
                    println!("{}", current_pixel_index);
                }
                self.canvas.set_draw_color(color(pixel_buffer[current_pixel_index]));
                let y = row as u32 * PIXEL_SIZE;
                let x = column as u32 * PIXEL_SIZE;
                let _ = self.canvas.fill_rect(Rect::new(x as i32, y as i32, PIXEL_SIZE, PIXEL_SIZE));
            }
        }
        self.canvas.present();
    }
}

fn color(pixel: bool) -> pixels::Color {
    if pixel {
        pixels::Color::RGB(255,255,255)
    } else {
        pixels::Color::RGB(0,0,0)
    }
}