use sdl2::keyboard::KeyboardState;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const ROWS: usize = 64;
const COLS: usize = 32;

pub struct Display {
    
    pub position: [bool; ROWS * COLS],
}

impl Display {
    pub fn new() -> Display {
        let array: [bool; ROWS * COLS] = [false; ROWS * COLS];
        Display {
            position: array,
        }
    }

    pub fn rows() -> usize {
        ROWS
    }

    pub fn cols() -> usize {
        COLS
    }

    pub fn display(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        
        let mut event_pump = sdl_context.event_pump().unwrap();

        'main: loop {
            // TODO: Add display drawing here
            canvas.set_draw_color(Color::RGB(255,255,255));
            canvas.clear();
            canvas.present();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    _ => {}
                }
            }
        }
    }
}