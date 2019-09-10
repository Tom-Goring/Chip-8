mod chip8;
mod instruction;

fn main() {
	let chip = chip8::Chip8::new();
	let ins = instruction::OpCodeInstruction::new(value: u16);
	
}
