#![allow(dead_code)]
#![allow(unused_mut)]

use crate::display;
//use crate::instruction;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

// TESTING

// TESTING

const MEMORY_SIZE: usize = 4 * 1024;
const NUM_STACK_FRAMES: usize = 16;
const NUM_GENERAL_REGS: usize = 16;
const NUM_KEYS: usize = 16;

// TODO: Finish Chip8 data structure implementation

pub struct Chip8 {
	regs: [u8; NUM_GENERAL_REGS],
	i_reg: u16, // address register
	sp: u8, // stack pointer
	pc: u16, // program counter
	memory: [u8; MEMORY_SIZE], // memory storage
	stack: [u16; NUM_STACK_FRAMES], // stack frames
	keyboard: [bool; NUM_KEYS], // 16 keys
	display: display::Display, // 64 * 32 display
	delay_timer: u8,
	sound_timer: u8,
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
			display: display::Display::new(),
		 }
	}

	pub fn run(&mut self) {
		let exit = false;
		let (parent_sender, thread_receiver): (Sender<bool>, Receiver<bool>) = mpsc::channel();
		let (thread_sender, parent_receiver): (Sender<bool>, Receiver<bool>) = mpsc::channel();


		let child = self.display.display(thread_sender, thread_receiver);


		// TODO: add a check for ending the display thread here

		loop {

			if parent_receiver.try_recv() == Ok(false) {
				break;
			}

			// game logic

			// parent_sender.send(true);
			// break;
		}

		println!("Game loop ended");
	}
}