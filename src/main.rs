#![allow(dead_code)]

mod chip8;
mod instruction;

fn main() {
	let raw = 0x8004;
	let ins = instruction::OpCodeInstruction::new(raw);
	println!("{:?}", ins.process_opcode());
}
