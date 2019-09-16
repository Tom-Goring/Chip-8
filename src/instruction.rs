#![allow(dead_code)]
// TODO: Finish instruction implementation

pub type Address = u16;
pub type Register = u8;

#[derive(Debug)]
pub enum Instruction {
	SYS(Address),
	CLS(),
	RET(),
	JMP(Address),
	CALL(Address),
	SEQB(Register, u8),
	SNEQB(Register, u8),
	SRER(Register, Register),
	LBR(Register, u8),
	ABR(Register, u8),
	LRR(Register, Register),
	OR(Register, Register),
	AND(Register, Register),
	XOR(Register, Register),
	ADD(Register, Register), // VF set to 1 if overflow
	SUB(Register, Register), // VF set to 1 if negative overflow
	SHR(Register), // Shift right 1 (binary divide by 2)
	SUBN(Register, Register), // 2nd REG - 1st REG set VF if result negative
	SHL(Register), // Shift left 1 (binary multiply by 2)
	SNE(Register, Register), // Skip PC if not equal
	LDI(Address), // Load i_reg with addr
	JPV0(Address), // JMP to address + V0
	RND(Register, u8), // Creates random number, AND's it with u8, then stores in Register
	DRW(Register, Register, u8), // reads n bytes from i_reg, then displays them on screen at co-ords (Vx, Vy). 
								 // Sprites are XOR'd onto the screen. Erased pixels cause VF t obe set to 1. 
								 // Sprites wrap around to opposite sides of the screen.
	SKP(Register), // Skips if key with value Register is pressed
	SKNP(Register), // Skips if key with value Register is not pressed
	LDDV(Register), // Value of delay timer is placed into Register
	LDK(Register), // Wait for key press then store value of key into Register
	LDVD(Register), // Set delay timer to value of Register
	LDST(Register), // Set sound timer to value of Register
	ADDI(Register), // Add Register to address register
	LDS(Register), // Address register set to location for the sprite corresponding to Register
	BCD(Register), // Store BCD representation of Register in memory locations I, I+1, I+2
	SR(Register), // Store Registers
	LR(Register) // Load Registers
}

#[derive(Debug)]
pub struct OpCodeInstruction {
	value: u16, // All instructions are 2 bytes long BigEndian style. The first byte should be loaded at even addresses in memory. 
				// Sprite data should be padded to maintain alignment.
}

impl OpCodeInstruction {
	pub fn new(value: u16) -> OpCodeInstruction {
		OpCodeInstruction { value }
	}

	pub fn process_opcode(&self) -> Option<Instruction> {

		let i = self.i();
		let nnn = self.nnn();
		let n = self.n();
		let x = self.x();
		let y = self.y();
		let kk = self.kk();

		match i {
			0x0 => {
				match kk {
					0xE0 => Some(Instruction::CLS()),
					0xEE => Some(Instruction::RET()),
					_ => None,
				}
			},
			0x1 => Some(Instruction::JMP(nnn)),
			0x2 => Some(Instruction::CALL(nnn)),
			0x3 => Some(Instruction::SEQB(n, kk)),
			0x4 => Some(Instruction::SNEQB(n, kk)),
			0x5 => Some(Instruction::SRER(x, y)),
			0x6 => Some(Instruction::LBR(x, kk)),
			0x7 => Some(Instruction::ABR(x, kk)),
			0x8 => {
				match n {
					0x0 => Some(Instruction::LRR(x, y)),
					0x1 => Some(Instruction::OR(x, y)),
					0x2 => Some(Instruction::AND(x, y)),
					0x3 => Some(Instruction::XOR(x, y)),
					0x4 => Some(Instruction::ADD(x, y)),
					0x5 => Some(Instruction::SUB(x, y)),
					0x6 => Some(Instruction::SHR(x)),
					0x7 => Some(Instruction::SUBN(x, y)),
					0xE => Some(Instruction::SHL(x)),
					_ => None,
				}
			}
			0x9 => Some(Instruction::SNE(x, y)),
			0xA => Some(Instruction::LDI(nnn)),
			0xB => Some(Instruction::JPV0(nnn)),
			0xC => Some(Instruction::RND(x, kk)),
			0xD => Some(Instruction::DRW(x, y, n)),
			0xE => {
				match kk {
					0x9E => Some(Instruction::SKP(x)),
					0xA1 => Some(Instruction::SKNP(x)),
					_ => None,
				}
			}
			0xF => {
				match kk {
					0x07 => Some(Instruction::LDDV(x)),
					0x0A => Some(Instruction::LDK(x)),
					0x15 => Some(Instruction::LDVD(x)),
					0x18 => Some(Instruction::LDST(x)),
					0x1E => Some(Instruction::ADDI(x)),
					0x29 => Some(Instruction::LDS(x)),
					0x33 => Some(Instruction::BCD(x)),
					0x55 => Some(Instruction::SR(x)),
					0x65 => Some(Instruction::LR(x)),
					_ => None,
				}
			}
			_ => None
		}
	}

	fn i(&self) -> u8 {
		((self.value & 0xF000) >> 12) as u8
	}

	fn nnn(&self) -> u16 {
		self.value & 0x0FFF
	}

	fn n(&self) -> u8 {
		(self.value & 0x000F) as u8
	}

	fn x(&self) -> u8 {
		((self.value & 0x0F00) >> 8) as u8
	}

	fn y(&self) -> u8 {
		((self.value & 0x00F0) >> 4) as u8
	}

	fn kk(&self) -> u8 {
		(self.value & 0x00FF) as u8
	}
}


#[cfg(test)]

// u16 == 39854 == 1001101110101110
// nnn == 2990  == 	   101110101110
// n   ==  14   ==        	   1110
// x   ==  11   ==     1011
// y   ==  10	==	        1010
// kk  ==  174  ==         10101110

mod tests {
	use super::*;

	#[test]
    fn i() {
        let x = 39854;
		let ins = OpCodeInstruction::new(x);
		let y = ins.i();
		assert!(y == 9);
    }

    #[test]
    fn nnn() {
        let x = 39854;
		let ins = OpCodeInstruction::new(x);
		let y = ins.nnn();
		assert!(y == 2990);
    }

	#[test]
	fn n() {
		let x = 39854;
		let ins = OpCodeInstruction::new(x);
		let y = ins.n();
		assert!(y == 14);
	}

	#[test]
	fn x() {
		let x = 39854;
		let ins = OpCodeInstruction::new(x);
		let y = ins.x();
		assert!(y == 11);
	}

	#[test]
	fn y() {
		let x = 39854;
		let ins = OpCodeInstruction::new(x);
		let y = ins.y();
		assert!(y == 10);
	}

	#[test]
	fn kk() {
		let x = 39854;
		let ins = OpCodeInstruction::new(x);
		let y = ins.kk();
		assert!(y == 174);
	}
}