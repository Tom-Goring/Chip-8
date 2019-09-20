extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use std::ops::BitXorAssign;

use wasm_bindgen::prelude::*;

use super::font::FONT_SET;
use super::instruction::{Instruction, OpCodeInstruction};

use rand;
use rand::Rng;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
    OFF = 0,
    ON = 1,
}

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const MEMORY_SIZE: usize = 4 * 1024;
const NUM_STACK_FRAMES: usize = 16;
const NUM_GENERAL_REGS: usize = 16;
const NUM_KEYS: usize = 16;

#[wasm_bindgen]
pub struct CPU {
    regs: [u8; NUM_GENERAL_REGS],
	i_reg: usize, // address register
	sp: usize, // stack pointer
	pc: usize, // program counter
	memory: [u8; MEMORY_SIZE], // memory storage
	stack: [u8; NUM_STACK_FRAMES], // stack frames
	delay_timer: u8,
	sound_timer: u8,
	display: Vec<Pixel>,
	keys: [bool; NUM_KEYS],
}

#[wasm_bindgen]
impl CPU {
    pub fn new() -> CPU {
		log!("CPU.new()");
		crate::utils::set_panic_hook();
		let mut memory = [0; MEMORY_SIZE];

		for (i, byte) in FONT_SET.iter().enumerate() {
			memory[i] = *byte;
		}

		let mut game_data = Vec::new();
		// file.read_to_end(&mut game_data).expect("Failure to read file.");

		game_data.push(0xA2);
		game_data.push(0x0A);
		game_data.push(0x60);
		game_data.push(0x0A);
		game_data.push(0x61);
		game_data.push(0x05);
		game_data.push(0xD0);
		game_data.push(0x17);
		game_data.push(0x12);
		game_data.push(0x08);

		// TODO: add a load function later
		for (i, &byte) in game_data.iter().enumerate() {
			let addr = 0x200 + i;
			if addr < 4096 {
				memory[0x200 + i] = byte;
			}
		}

		memory[0x20A] = 0x7C;
		memory[0x20B] = 0x40;
		memory[0x20C] = 0x40;
		memory[0x20D] = 0x7C;
		memory[0x20E] = 0x40;
		memory[0x20F] = 0x40;
		memory[0x210] = 0x7C;

		let mut display = vec![Pixel::OFF; CHIP8_HEIGHT as usize * CHIP8_WIDTH as usize];

		CPU {
			regs: [0; NUM_GENERAL_REGS],
			i_reg: 0,
			delay_timer: 0,
			sound_timer: 0,
			sp:  0,
			pc: 0x200,
			memory,
			stack: [0; NUM_STACK_FRAMES],
			display,
			keys: [false; NUM_KEYS],
		 }
	}

	pub fn height(&self) -> u32 {
		CHIP8_HEIGHT
	}

	pub fn width(&self) -> u32 {
		CHIP8_WIDTH
	}

	pub fn pixels(&self) -> *const Pixel {
		self.display.as_ptr()
	}

	pub fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width() + column) as usize
	}

	pub fn tick(&mut self) {
		
		let instr = self.fetch_instruction();
		log!("{:?}", instr);
		self.execute_instruction(instr);
	}
}

impl CPU {
	fn fetch_instruction(&self) -> Instruction {
		let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
		OpCodeInstruction::new(opcode).process_opcode().unwrap()
	}

	fn execute_instruction(&mut self, instruction: Instruction) {
		match instruction {

			// 00E0 - Clear Screen
			Instruction::CLS() => {
				// TODO: re-implement CLS
			},

			// 00EE - Return from subroutine
			Instruction::RET() => {
				self.sp -= 1;
				self.pc = self.stack[self.sp] as usize;
				self.pc += 2;
			},

			// 1NNN - Jumps to address NNN
			Instruction::JMP(addr) => {
				self.pc = (addr) as usize;
			},

			// 2NNN - Calls subroutine at NNN
			Instruction::CALL(addr) => {
				self.stack[self.sp] = self.pc as u8;
				self.sp += 1;
				self.pc = addr as usize;
			},

			// 3XNN - Skips next instruction if VX == NN
			Instruction::SEQB(reg, value) => {
				if self.get_register(reg) == value {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			// 4XNN - Skips next instruction if VX != NN
			Instruction::SNEQB(reg, value) => {
				if self.get_register(reg) != value {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			// 5XY0 - Skip next instruction if VX == VY
			Instruction::SRER(reg1, reg2) => {
				if self.get_register(reg1) == self.get_register(reg2) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			// 6XNN - Sets VX to NN
			Instruction::LBR(reg, value) => {
				self.set_register(reg, value);
				self.pc += 2;
			},

			// 7XNN - Ads NN to VX
			Instruction::ABR(reg, value) => {
				self.set_register(reg, value.wrapping_add(self.get_register(reg)));
				self.pc += 2;
			},

			// 8XY0 - Sets VX to value of VY
			Instruction::LRR(reg1, reg2) => {
				self.set_register(reg1, self.get_register(reg2));
				self.pc += 2;
			},

			// 8XY1 - Sets VX to (VX | VY)
			Instruction::OR(reg1, reg2) => {
				let value = self.get_register(reg1) | self.get_register(reg2);
				self.set_register(reg1, value);
				self.pc += 2;
			},

			// 8XY2 - Sets VX to (VX & VY)
			Instruction::AND(reg1, reg2) =>  {
				let value = self.get_register(reg1) & self.get_register(reg2);
				self.set_register(reg1, value);
				self.pc += 2;
			},

			// 8XY3 - Sets VX to (VX ^ VY)
			Instruction::XOR(reg1, reg2) => {
				let value = self.get_register(reg1) ^ self.get_register(reg2);
				self.set_register(reg1, value);
				self.pc += 2;
			},

			// 8XY4 - Adds VY to VX, sets VF to carry
			Instruction::ADD(reg1, reg2) => {
				let sum = self.get_register(reg1) as u16 + self.get_register(reg2) as u16;
				self.set_register(0xF, (sum > 255) as u8);
				self.set_register(reg1, sum as u8);
				self.pc += 2;
			},

			// 8XY5 - Subtracts VY from VX. VF set to borrow.
			Instruction::SUB(reg1, reg2) => {
				self.set_register(0xF, (self.get_register(reg1) > self.get_register(reg2)) as u8);
				self.set_register(reg1, self.get_register(reg1).wrapping_sub(self.get_register(reg2)));
				self.pc += 2;
			},

			// 8XY6 - Shifts value of reg right by one. VF set to least sig bit before shift.
			Instruction::SHR(reg) => {
				// The result of an and with 0b1 is the state of the rightmost bit
				self.set_register(0xF, self.get_register(reg) & 0b1);
				self.set_register(reg, self.get_register(reg) >> 1);
				self.pc += 2;
			},

			// 8XY7 - Sets VX to VY - VX. VF set to !borrow.
			Instruction::SUBN(reg1, reg2) => {
				if self.get_register(reg2) > self.get_register(reg1) {
					self.set_register(0xF, 0);
				} else {
					self.set_register(0xF, 1);
				}
				self.set_register(reg2, self.get_register(reg2).wrapping_sub(self.get_register(reg1)));
				self.pc += 2;
			},

			// 8XYE - Shifts value of reg left by one. VF set to most sig bit before shift.
			Instruction::SHL(reg) => {
				// Moving a u8 right 7 will leave it as a binary 0/1 only
				self.set_register(0xF, self.get_register(reg) >> 7);
				self.set_register(reg, self.get_register(reg) << 1);
				self.pc += 2;
			},

			// 9XY0 - Skips next instruction if VX != VY
			Instruction::SNE(reg1, reg2) => {
				if self.get_register(reg1) != self.get_register(reg2) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			// ANNN - Sets I to NNN
			Instruction::LDI(addr) => {
				self.i_reg = addr as usize;
				self.pc += 2;
			},

			// BNNN - Jumps to address NNN plus V0
			Instruction::JPV0(addr) => {
				self.pc = (addr as usize) + (self.get_register(0) as usize);
			},

			// CXNN Sets VX to random number masked by NN
			Instruction::RND(reg, nn) => {
				let mut rng = rand::thread_rng();
				self.set_register(reg, rng.gen::<u8>() & nn);
				self.pc += 2;
			},

			// DXYN = Draws sprite at (VX, VY) with width 8 and height N. Detects collision.
			Instruction::DRW(reg1, reg2, num_bytes) => {
				log!("DRAW");
				let x = self.get_register(reg1);
				let y = self.get_register(reg2);

				log!("{}", self.i_reg);

				for x in 520..520+0x200 {
					log!("{}", self.memory[self.i_reg + x]);
				}

				for index in 0..num_bytes {
					self.set_register(0xF, 0);
					let y = (y + index) as usize % CHIP8_HEIGHT as usize;

					for bit in 0..8 {
						let x = (x as u16 + bit as u16) as usize % CHIP8_WIDTH as usize;

						let itx = self.get_index(y as u32, x as u32);

						let pixel_to_display = (self.memory[self.i_reg + index as usize] >> (7 - bit)) & 1; // gets the specific bit of the current byte we're looking at

						if self.display[itx] == Pixel::ON {
							self.set_register(0xF, self.get_register(0xF) | pixel_to_display & 1);
						}
						else {
							self.set_register(0xF, self.get_register(0xF) | pixel_to_display & 0);
						}

						if self.display[itx] == Pixel::OFF && pixel_to_display == 1 {
							self.display[itx] = Pixel::ON;
							log!("Turned pixel ON");
						} 
						else if self.display[itx] == Pixel::ON && pixel_to_display == 0 {
							self.display[itx] = Pixel::OFF;
							log!("Turned pixel OFF");
						}
					}
				}
				self.pc += 2;
			},

			// EX9E - Skips next instruction if key in VX isn't pressed.
			Instruction::SKP(reg) => {
				if self.keys[self.get_register(reg) as usize]{
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			// EXA1 - Skips next instruction if key in VX isn't pressed.
			Instruction::SKNP(reg) => {
				if !self.keys[self.get_register(reg) as usize]{
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			// FX07 - Sets VX to value of delay timer.
			Instruction::LDDV(reg) => {
				self.set_register(reg, self.delay_timer);
				self.pc += 2;
			},

			// FX0A - Key press is awaited, then stored in VX.
			Instruction::LDK(reg) => {
				let pressed_key;
				'keyloop: loop {
					for key in self.keys.iter() {
						if *key {
							pressed_key = *key;
							break 'keyloop;
						}
					}
				}
				self.set_register(reg, pressed_key as u8);
				self.pc += 2;
			},

			// FX15 - Set delay timer to VX.
			Instruction::LDVD(reg) => {
				self.delay_timer = self.get_register(reg);
				self.pc += 2;
			},

			// FX18 - Set sound timer to VX.
			Instruction::LDST(reg) => {
				self.sound_timer = self.get_register(reg);
				self.pc += 2;
			},

			// FX1E - ADd VX to i_reg.
			Instruction::ADDI(reg) => {
				self.i_reg += self.get_register(reg) as usize;
				self.pc += 2;
			},

			// FX29 - Sets i_reg to location of sprite in VX.
			Instruction::LDS(reg) => {
				let sprite = self.get_register(reg);
				self.i_reg = (sprite * 5) as usize;
				self.pc += 2;
			},

			// FX33 - Store binary-coded decimal representation of VX at i, i+1, and i+3.
			Instruction::BCD(reg) => {
				self.memory[self.i_reg] = self.get_register(reg) / 100;
				self.memory[self.i_reg + 1] = (self.get_register(reg) % 100) / 10;
				self.memory[self.i_reg + 2] = self.get_register(reg) % 10;
				self.pc += 2;
			},

			// FX55 - Stores V0 through VX in memory starting at i_reg.
			Instruction::SR(reg) => {
				for x in 0..=reg {
					let value = self.get_register(x);
					self.memory[self.i_reg + x as usize] = value;
				}
				self.i_reg += reg as usize + 1;
				self.pc += 2;
			},

			// FX66 - Loads V0 through VX from memory starting at i_reg.
			Instruction::LR(reg) => {
				for x in 0..=reg {
					self.set_register(x, self.memory[self.i_reg + x as usize]);
				}
				self.i_reg += reg as usize + 1;
				self.pc += 2;
			},
		}
	}

	fn get_register(&self, reg: u8) -> u8 {
		self.regs[reg as usize] as u8
	}

	fn set_register(&mut self, reg: u8, value: u8) {
		self.regs[reg as usize] = value;
	}
}