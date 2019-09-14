#![allow(dead_code)]

mod instruction;
mod chip8;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {

	let file_name = env::args().nth(1).expect("Expected a valid game name as argument!");
	let mut file = File::open(file_name).expect("There was an issue opening the game file.");
	let mut game_data = Vec::new();
	file.read_to_end(&mut game_data).expect("Failure to read file.");

	let mut chip8 = chip8::Chip8::new(game_data);
	chip8.run();
}