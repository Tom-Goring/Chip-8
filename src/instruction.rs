#![allow(dead_code)]
// TODO: Finish instruction implementation

pub type Address = u16;
pub type Register = u8;

pub enum Instruction {
	SYS(Address),
	ClearScreen,
	Return,
	JMP(Address),
	CALL(Address),
	SkipIfEqualToByte(Register, u8),
	SkipIfIfNotEqualToByte(Register, u8),
	SkipIfRegisterEqualToRegister(Register, Register),
	LoadByteToRegister(Register, u8),
	AddByteToRegister(Register, u8),
	LoadRegisterToRegister(Register, Register),
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
	SR, // Store Registers
	LR // Load Registers
}

pub struct OpCodeInstruction {
	value: u16, // All instructions are 2 bytes long BigEndian style. The first byte should be loaded at even addresses in memory. 
				// Sprite data should be padded to maintain alignment.
}

impl OpCodeInstruction {
	pub fn new(value: u16) -> OpCodeInstruction {
		OpCodeInstruction { value: value }
	}

	pub fn process_opcode(&self) -> Option<Instruction> {

		let nnn = self.nnn();
		let n = self.n();
		let x = self.x();
		let y = self.y();
		let kk = self.kk();

		match nnn {
			0x0 => {
				match kk {
					0xE0 => Some(Instruction::ClearScreen),
					0xEE => Some(Instruction::Return),
					_ => None,
				}
			}

			
			_ => None,

		}
	}

	pub fn nnn(&self) -> u16 {
		self.value & 0x0FFF
	}

	fn n(&self) -> u8 {
		(self.value & 0x000F) as u8
	}

	fn x(&self) -> u8 {
		(self.value & 0x0F00) as u8
	}

	fn y(&self) -> u8 {
		(self.value & 0x00F0) as u8
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