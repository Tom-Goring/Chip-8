// TODO: Finish instruction implementation

pub enum Instruction {
	SYS(addr),
	ClearScreen,
	Return,
	JMP(addr),
	CALL(addr),
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
	LDI(addr), // Load i_reg with addr
	JPV0(addr), // JMP to address + V0
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
	pub fn new(value: u16) -> Instruction {
		Instruction { value: value }
	}

	pub fn processOpCode(&self) -> Option<OpCodeInstruction> {
		// TODO: look at how 0x0 works
		match self.value {
			0x1 => Some(Instruction::JMP(self.value)),
			0x2 => Some(Instruction::CALL(self.value)),


		}
	}

	fn addr(&self) -> u8 {
		
	}
}

