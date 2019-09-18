#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use super::drivers::{DisplayDriver};
use super::instruction::{Instruction, OpCodeInstruction};
use super::font::FONT_SET;

use rand;
use rand::Rng;

use std::thread;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
	display: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
	keys: [bool; NUM_KEYS],
	waiting_on_keypress: bool,
}

impl Chip8 {
	pub fn new(program: Vec<u8>) -> Chip8 {
		let mut memory = [0; MEMORY_SIZE];

		for (i, byte) in FONT_SET.iter().enumerate() {
			memory[i] = byte.clone();
		}

		for (i, &byte) in program.iter().enumerate() {
			let addr = 0x200 + i;
			if addr < 4096 {
				memory[0x200 + i] = byte;
			}
		}

		let mut display = [[0 as u8; CHIP8_WIDTH]; CHIP8_HEIGHT];

		for y in 0..CHIP8_HEIGHT {
			for x in 0..CHIP8_WIDTH {
				display[y][x] = 0;
			}
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
			display: display,
			keys: [false; NUM_KEYS],
			waiting_on_keypress: false
		 }
	}

	pub fn run(&mut self) {
		let sdl_context = sdl2::init().unwrap();
		let mut display_driver = DisplayDriver::new(&sdl_context);
		let mut events = sdl_context.event_pump().unwrap();

		display_driver.draw(&self.display);

		'main: loop { // fetch decode execute loop

			for event in events.poll_iter() {
				match event {
					Event::Quit {..} => break 'main,
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                	},
					_ => {}
				}
			}

			let instr = self.fetch_instruction();
			println!("{:?}", instr);
			self.execute_instruction(instr);

			display_driver.draw(&self.display); // consider adding check to see if display has changed
												// to try getting rid of annoying flashing

			println!("Instruction executed");

			thread::sleep(Duration::from_millis(2));
		}
	}

	fn tick(&mut self) {
		let instruction = self.fetch_instruction();
		self.execute_instruction(instruction);

	}

	fn fetch_instruction(&self) -> Instruction {
		let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
		println!("OpCode: {:X?}", opcode);
		OpCodeInstruction::new(opcode).process_opcode().unwrap()
	}

	fn execute_instruction(&mut self, instruction: Instruction) {
		match instruction {

			Instruction::CLS() => {
				for row in 0..CHIP8_HEIGHT {
					for column in 0..CHIP8_WIDTH {
						self.display[row][column] = 0;
					}
				}
				self.pc += 2;
			},

			Instruction::RET() => {
				self.pc = self.stack[self.sp];
				self.sp -= 1;
				self.pc += 2;
			},

			Instruction::JMP(addr) => {
				self.pc = (addr) as usize;
			},

			Instruction::CALL(addr) => {
				self.sp += 1;
				self.stack[self.sp] = self.pc;
				self.pc = addr as usize;
			},

			Instruction::SEQB(reg, value) => {
				if self.get_register(reg) == value {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			Instruction::SNEQB(reg, value) => {
				if self.get_register(reg) != value {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			Instruction::SRER(reg1, reg2) => {
				if self.get_register(reg1) == self.get_register(reg2) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			Instruction::LBR(reg, value) => {
				self.set_register(reg, value);
				self.pc += 2;
			},

			Instruction::ABR(reg, value) => {
				self.set_register(reg, value.wrapping_add(self.get_register(reg)));
				self.pc += 2;
			},

			Instruction::LRR(reg1, reg2) => {
				self.set_register(reg1, self.get_register(reg2));
				self.pc += 2;
			},

			Instruction::OR(reg1, reg2) => {
				let value = self.regs[reg1 as usize] | self.regs[reg2 as usize];
				self.set_register(reg1, value);
				self.pc += 2;
			},

			Instruction::AND(reg1, reg2) =>  {
				let value = self.regs[reg1 as usize] & self.regs[reg2 as usize];
				self.set_register(reg1, value);
				self.pc += 2;
			},

			Instruction::XOR(reg1, reg2) => {
				let value = self.regs[reg1 as usize] ^ self.regs[reg2 as usize];
				self.set_register(reg1, value);
				self.pc += 2;
			},

			Instruction::ADD(reg1, reg2) => {
				let sum = self.get_register(reg1) as u16 + self.get_register(reg2) as u16;
				if sum > 255 {
					self.set_register(0xF, 1)
				}
				self.set_register(reg1, sum as u8);
				self.pc += 2;
			},

			Instruction::SUB(reg1, reg2) => {
				if self.get_register(reg1) > self.get_register(reg2) {
					self.set_register(0xF, 1)
				}
				self.set_register(reg1, self.get_register(reg1).wrapping_sub(self.get_register(reg2)));
				self.pc += 2;
			},

			Instruction::SHR(reg) => {
				if self.get_register(reg) & 0b1 == 1 { // The result of an and with 0b1 is the state of the rightmost bit
					self.set_register(0xF, 1)
				}
				self.set_register(reg, self.get_register(reg) >> 1);
				self.pc += 2;
			},

			Instruction::SUBN(reg1, reg2) => {
				if self.get_register(reg2) > self.get_register(reg1) {
					self.set_register(0xF, 1)
				}
				self.set_register(reg2, self.get_register(reg2) - self.get_register(reg1));
				self.pc += 2;
			},

			Instruction::SHL(reg) => {
				
				if self.get_register(reg) >> 7 == 1 { // Moving a u8 right 7 will leave it as a binary 0/1 only
					self.set_register(0xF, 1)
				}
				self.set_register(reg, self.get_register(reg) << 1);
				self.pc += 2;
			},

			Instruction::SNE(reg1, reg2) => {
				if self.get_register(reg1) != self.get_register(reg2) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			Instruction::LDI(addr) => {
				self.i_reg = addr as usize;
				self.pc += 2;
			},

			Instruction::JPV0(addr) => {
				self.pc = (addr as usize) + (self.regs[0] as usize);
			},

			Instruction::RND(reg, num_bytes) => {
				let mut rng = rand::thread_rng();
				self.set_register(reg, rng.gen::<u8>() & num_bytes);
				self.pc += 2;
			},

			Instruction::DRW(reg1, reg2, num_bytes) => {
				let x = self.get_register(reg1);
				let y = self.get_register(reg2);
				let first = self.i_reg;
				let last = first + num_bytes as usize;

				for index in 0..num_bytes {
					let byte = self.memory[self.i_reg + index as usize];
					let y = (y as usize + index as usize) as usize % CHIP8_HEIGHT; // should wrap back to top of display?
					for bit in 0..8 {
						let x = (x + bit) as usize % CHIP8_WIDTH; // should wrap back to start of line?
						let pixel_active_on_display =  self.display[y][x]; // get status of pixel on display for collision detection
						let pixel_to_display = byte >> (7 - bit) & 1; // gets the specific bit of the current byte we're looking at
						self.set_register(0xF, pixel_active_on_display & pixel_to_display); // set register 15 if a collision is detected
						self.display[y][x] ^= pixel_to_display;
					}
				}
				self.pc += 2;
			},

			Instruction::SKP(reg) => {
				if self.keys[reg as usize] == true {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			Instruction::SKNP(reg) => {
				if self.keys[reg as usize] != true {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			Instruction::LDDV(reg) => {
				self.set_register(reg, self.delay_timer);
				self.pc += 2;
			},

			Instruction::LDK(reg) => {
				let mut pressed_key;
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

			Instruction::LDVD(reg) => {
				self.set_register(reg, self.delay_timer);
				self.pc += 2;
			},

			Instruction::LDST(reg) => {
				self.set_register(reg, self.sound_timer);
				self.pc += 2;
			},

			Instruction::ADDI(reg) => {
				self.i_reg += self.get_register(reg) as usize;
				self.pc += 2;
			},

			Instruction::LDS(reg) => {
				let sprite = self.get_register(reg);
				self.i_reg = (sprite * 5) as usize;
				self.pc += 2;
			},

			Instruction::BCD(reg) => {
				self.memory[self.i_reg] = self.get_register(reg) / 100;
				self.memory[self.i_reg + 1] = (self.get_register(reg) % 100) / 10;
				self.memory[self.i_reg + 2] = self.get_register(reg) % 10;
				self.pc += 2;
			},

			Instruction::SR(reg) => {
				for x in 0..reg + 1 {
					let value = self.get_register(reg);
					self.memory[self.i_reg + x as usize] = value;
				}
				self.pc += 2;
			},

			Instruction::LR(reg) => {
				for x in 0..reg + 1 {
					self.set_register(x, self.memory[self.i_reg + x as usize]);
				}
				self.pc += 2;
			},

			_ => {}
		}
	}

	fn get_register(&self, reg: u8) -> u8 {
		self.regs[reg as usize] as u8
	}

	fn set_register(&mut self, reg: u8, value: u8) {
		self.regs[reg as usize] = value;
	}
}