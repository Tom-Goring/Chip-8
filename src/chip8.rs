#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::drivers::{DisplayDriver};
use super::instruction::{Instruction, OpCodeInstruction};

use crate::CHIP8_WIDTH;
use crate::CHIP8_HEIGHT;

const MEMORY_SIZE: usize = 4 * 1024;
const NUM_STACK_FRAMES: usize = 16;
const NUM_GENERAL_REGS: usize = 16;
const NUM_KEYS: usize = 16;

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
			pc: 0,
			memory,
			stack: [0; NUM_STACK_FRAMES],
			keyboard: [false; NUM_KEYS],
			buffer: [false; CHIP8_WIDTH * CHIP8_HEIGHT]
		 }
	}

	pub fn run(&mut self) {
		let sdl_context = sdl2::init().unwrap();
		let mut display_driver = DisplayDriver::new(&sdl_context);

		self.buffer[10] = true;
		display_driver.draw(&self.buffer);

		let instr = self.fetch_instruction();
		println!("{:?}", instr);
		self.execute_instruction(instr);

		display_driver.draw(&self.buffer);

		println!("Instruction executed");
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

			Instruction::CLS() => {
				for index in 0..CHIP8_WIDTH * CHIP8_HEIGHT {
					self.buffer[index] = false;
				}
			},

			Instruction::RET() => {

			},

			Instruction::JMP(addr) => {

			},

			Instruction::CALL(addr) => {

			},

			Instruction::SEQB(reg, value) => {

			},

			Instruction::SNEQB(reg, value) => {

			},

			Instruction::SRER(reg1, reg2) => {

			},

			Instruction::LBR(reg, value) => {

			},

			Instruction::ABR(reg, value) => {

			},

			Instruction::LRR(reg1, reg2) => {

			},

			Instruction::OR(reg1, reg2) => {

			},

			Instruction::AND(reg1, reg2) =>  {

			},

			Instruction::XOR(reg1, reg2) => {

			},

			Instruction::ADD(reg1, reg2) => {

			},

			Instruction::SUB(reg1, reg2) => {

			},

			Instruction::SHR(reg) => {

			},

			Instruction::SUBN(reg1, reg2) => {

			},

			Instruction::SHL(reg) => {

			},

			Instruction::SNE(reg1, reg2) => {

			},

			Instruction::LDI(addr) => {

			},

			Instruction::JPV0(addr) => {

			},

			Instruction::RND(reg, value) => {

			},

			Instruction::DRW(reg1, reg2, value) => {

			},

			Instruction::SKP(reg) => {

			},

			Instruction::SKNP(reg) => {

			},

			Instruction::LDDV(reg) => {

			},

			Instruction::LDK(reg) => {

			},

			Instruction::LDVD(reg) => {

			},

			Instruction::LDST(reg) => {

			},

			Instruction::ADDI(reg) => {

			},

			Instruction::LDS(reg) => {

			},

			Instruction::BCD(reg) => {

			},

			Instruction::SR(reg) => {

			},

			Instruction::LR(reg) => {

			},

			_ => {}
		}
		self.pc += 2;
	}
}