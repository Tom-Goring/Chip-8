#![allow(dead_code)]

mod instruction;
mod display;
mod chip8;


fn main() {

	let chip8 = chip8::Chip8::new();
	chip8.activate_display();
}