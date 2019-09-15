#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::drivers::{DisplayDriver};
use super::instruction::{Instruction, OpCodeInstruction};

use crate::CHIP8_WIDTH;
use crate::CHIP8_HEIGHT;

const CLOCK_RATE: f64 = 600.0;
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
	i_reg: usize, // address register
	sp: usize, // stack pointer
	pc: usize, // program counter
	memory: [u8; MEMORY_SIZE], // memory storage
	stack: [usize; NUM_STACK_FRAMES], // stack frames
	keyboard: [bool; NUM_KEYS], // 16 keys
	delay_timer: u8,
	sound_timer: u8,
	buffer: [bool; CHIP8_WIDTH * CHIP8_HEIGHT],
}

impl Chip8 {
	pub fn new(program: Vec<u8>) -> Chip8 {
		let mut memory = [0; MEMORY_SIZE];
		for (i, byte) in program.iter().enumerate() {
			memory[i] = byte.clone();
		}

		Chip8 {
			regs: [0; NUM_GENERAL_REGS],
			i_reg: 0,
			delay_timer: 0,
			sound_timer: 0,
			sp:  0,
			pc: 0x200,
			memory,
			stack: [0; NUM_STACK_FRAMES],
			keyboard: [false; NUM_KEYS],
			buffer: [false; CHIP8_WIDTH * CHIP8_HEIGHT]
		 }
	}

	pub fn run(&mut self) {
		let sdl_context = sdl2::init().unwrap();
		let mut display_driver = DisplayDriver::new(&sdl_context);

		let instr = self.fetch_instruction();
		println!("{:?}", instr);
	}

	fn tick(&mut self) {
		let instruction = self.fetch_instruction();
		self.execute_instruction(instruction);

	}

	fn fetch_instruction(&self) -> Instruction {
		let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
		OpCodeInstruction::new(opcode).process_opcode().unwrap()
	}

	fn execute_instruction(&mut self, instruction: Instruction) {
		match instruction {

			Instruction::ClearScreen => {

			},

			Instruction::Return => {

			},

			Instruction::JMP(addr) => {

			},

			Instruction::CALL(addr) => {

			},

			Instruction::SkipIfEqualToByte(reg, value) => {

			},

			Instruction::SkipIfIfNotEqualToByte(reg, value) => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			Instruction:: => {

			},

			

			_ => {}
		}
		self.pc += 2;
	}
}