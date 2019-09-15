#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate sdl2;

use std::env;
use std::fs::File;
use std::io::Read;

mod drivers;
mod instruction;
mod chip8;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_MEM: usize = 4096;

fn main() {
    let file_name = env::args().nth(1).expect("Expected a valid game name as argument!");
	let mut file = File::open(file_name).expect("There was an issue opening the game file.");
	let mut game_data = Vec::new();
	file.read_to_end(&mut game_data).expect("Failure to read file.");

    let mut chip8 = chip8::Chip8::new(game_data);
    chip8.run();
}