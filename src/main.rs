#![allow(dead_code)]

mod instruction;
mod display;
mod chip8;

use std::{thread, time};

fn main() {

	let mut chip8 = chip8::Chip8::new();
	chip8.run();
}