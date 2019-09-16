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
				self.pc = self.stack[self.sp] - 2;
				self.sp -= 1;
			},

			Instruction::JMP(addr) => {
				self.pc = addr as usize;
			},

			Instruction::CALL(addr) => {
				self.sp += 1;
				self.stack[self.sp] = self.pc;
				self.pc = addr as usize;
			},

			Instruction::SEQB(reg, value) => {
				if self.read_register(reg) == value {
					self.pc += 2;
				}
			},

			Instruction::SNEQB(reg, value) => {
				if self.read_register(reg) != value {
					self.pc += 2;
				}
			},

			Instruction::SRER(reg1, reg2) => {
				if self.read_register(reg1) == self.read_register(reg2) {
					self.pc += 2;
				}
			},

			Instruction::LBR(reg, value) => {
				self.change_register(reg, value);
			},

			Instruction::ABR(reg, value) => {
				self.change_register(reg, self.read_register(reg) + value);
			},

			Instruction::LRR(reg1, reg2) => {
				self.change_register(reg1, self.read_register(reg2));
			},

			Instruction::OR(reg1, reg2) => {
				let value = self.regs[reg1 as usize] | self.regs[reg2 as usize];
				self.change_register(reg1, value);
			},

			Instruction::AND(reg1, reg2) =>  {
				let value = self.regs[reg1 as usize] & self.regs[reg2 as usize];
				self.change_register(reg1, value);
			},

			Instruction::XOR(reg1, reg2) => {
				let value = self.regs[reg1 as usize] ^ self.regs[reg2 as usize];
				self.change_register(reg1, value);
			},

			Instruction::ADD(reg1, reg2) => {
				let sum = self.regs[reg1] + self.regs[reg2 as usize];
				if sum > 255 {
					self.regs[0xF] = 1;
				}
				self.regs[reg1 as usize] = (sum << 8) as u8;
			},

			Instruction::SUB(reg1, reg2) => {
				if self.regs[reg1 as usize] > self.regs[reg2 as usize] {
					self.regs[0xF] = 1;
				}
				self.regs[reg1 as usize] = self.regs[reg1 as usize] - self.regs[reg2 as usize];
			},

			Instruction::SHR(reg) => {
				if self.regs[reg as usize] & 0b1 == 1 { // The result of an and with 0b1 is the state of the rightmost bit
					self.regs[0xF] = 1;
				}
				self.regs[reg as usize] >> 1;
			},

			Instruction::SUBN(reg1, reg2) => {
				if self.regs[reg2 as usize] > self.regs[reg1 as usize] {
					self.regs[0xF] = 1;
				}
				self.regs[reg2 as usize] = self.regs[reg2 as usize] - self.regs[reg1 as usize];
			},

			Instruction::SHL(reg) => {
				if self.regs[reg as usize] >> 7 == 1 { // Moving a u8 right 7 will leave it as a binary 0/1 only
					self.regs[0xF] = 1;
				}
				self.regs[reg as usize] << 1;
			},

			Instruction::SNE(reg1, reg2) => {
				if self.regs[reg1 as usize] != self.regs[reg2 as usize] {
					self.pc += 2;
				}
			},

			Instruction::LDI(addr) => {
				self.i_reg = addr as usize;
			},

			Instruction::JPV0(addr) => {
				self.pc = (addr as usize) + (self.regs[0] as usize);
			},

			Instruction::RND(reg, value) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::DRW(reg1, reg2, value) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::SKP(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::SKNP(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::LDDV(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::LDK(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::LDVD(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::LDST(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::ADDI(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::LDS(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::BCD(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::SR(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			Instruction::LR(reg) => {
				panic!("{:?} not currently implemented!", instruction);
			},

			_ => {}
		}
		self.pc += 2;
	}

	fn read_register(&self, reg: u8) -> u8 {
		self.regs[reg as usize] as u8
	}

	fn change_register(&self, reg: u8, value: u8) {
		self.regs[reg as usize] = value;
	}
}