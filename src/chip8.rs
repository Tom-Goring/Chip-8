#![allow(dead_code)]
#![allow(unused_mut)]

//use crate::instruction;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;

const MEMORY_SIZE: usize = 4 * 1024;
const NUM_STACK_FRAMES: usize = 16;
const NUM_GENERAL_REGS: usize = 16;
const NUM_KEYS: usize = 16;
const PIXEL_SIZE: u32 = 20;
const COLS: usize = 64;
const ROWS: usize = 32;

// TODO: Finish Chip8 data structure implementation

pub struct Chip8 {
	regs: [u8; NUM_GENERAL_REGS],
	i_reg: u16, // address register
	sp: u8, // stack pointer
	pc: u16, // program counter
	memory: [u8; MEMORY_SIZE], // memory storage
	stack: [u16; NUM_STACK_FRAMES], // stack frames
	keyboard: [bool; NUM_KEYS], // 16 keys
	delay_timer: u8,
	sound_timer: u8,
	buffer: [bool; ROWS * COLS],
}

impl Chip8 {
	pub fn new() -> Chip8 {
		let mut memory = [0; MEMORY_SIZE];
		 Chip8 {
			regs: [0; NUM_GENERAL_REGS],
			i_reg: 0,
			delay_timer: 0,
			sound_timer: 0,
			sp:  0,
			pc: 0,
			memory,
			stack: [0; NUM_STACK_FRAMES],
			keyboard: [false; NUM_KEYS],
			buffer: [false; COLS * ROWS]
		 }
	}

	pub fn run(&mut self) {

		let window_height = 20 * COLS as u32;
        let window_width = 20 * ROWS as u32;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8", window_height, window_width)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        
        let mut event_pump = sdl_context.event_pump().unwrap();

        'main: loop {
			self.buffer[50] = true;
            canvas.set_draw_color(Color::RGB(255,255,255));
            canvas.clear();

            let mut black = false;
            
            for row in 0..ROWS {
                black = !black;
                for col in 0..COLS {
                    let x = col as i32 * PIXEL_SIZE as i32;
                    let y = row as i32 * PIXEL_SIZE as i32;
                    let rect = Rect::new(x, y, PIXEL_SIZE, PIXEL_SIZE);

                    if self.buffer[col+row] {
						canvas.set_draw_color(Color::RGB(0,0,0));
					} else {
						canvas.set_draw_color(Color::RGB(255,255,255));
					}

                    let _ = canvas.fill_rect(rect);
                }
            }

            canvas.present();

            for event in event_pump.poll_iter() {
                if let Event::Quit {..} = event { 
                    break 'main 
                }
            }
        }

		println!("Game loop ended");
	}
}