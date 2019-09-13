#![allow(dead_code)]

extern crate sdl2; 

use sdl2::keyboard::KeyboardState;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
   	let sdl_context = sdl2::init().unwrap();
   	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Chip-8", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	

	let mut rect = Rect::new(10, 10, 10, 10);
	
	let mut event_pump = sdl_context.event_pump().unwrap();

	'main: loop {
		for event in event_pump.poll_iter() {

			canvas.set_draw_color(Color::RGB(0,0,0));
			canvas.clear();
			canvas.present();

			let mut scanCodes = KeyboardState::scancodes(e: event_pump);

			match event {

				Event::Quit {..} => break 'main,

				Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
					rect.y -= 10;
				}

				Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
					rect.y += 10;
				}

				Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
					rect.x -= 10;
				}

				Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
					rect.x += 10;
				}

				_ => {}
			}

			canvas.set_draw_color(Color::RGB(255,255,255));
			canvas.fill_rect(rect);
			canvas.present();
		}
	}
		// stuff
}