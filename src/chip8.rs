const MEMORY_SIZE: usize = 4 * 1024;
const NUM_STACK_FRAMES: usize = 16;
const NUM_GENERAL_REGS: usize = 16;
const NUM_KEYS: usize = 16;
const DISPLAY_SIZE: usize = 64 * 32;

// TODO: Finish Chip8 data structure implementation

pub struct Chip8 {
	regs: [u8; NUM_GENERAL_REGS],
	i_reg: u16, // address register
	sp: u8, // stack pointer
	pc: u16, // program counter
	memory: [u8; MEMORY_SIZE], // memory storage
	stack: [u16; NUM_STACK_FRAMES], // stack frames
	keyboard: [bool; NUM_KEYS], // 16 keys
	display: [bool; DISPLAY_SIZE], // 64 * 32 display
	delay_timer: u8,
	sound_timer: u8,
}

impl Chip8 {
	pub fn new() -> Chip8 {
		let mut memory = [0; MEMORY_SIZE];
		 Chip8 {
			regs: [0; NUM_GENERAL_REGS],
			i_reg: 0,
			delay_timer: 0,
			sound_timer: 0,
			sp:  0,
			pc: 0,
			memory: memory,
			stack: [0; NUM_STACK_FRAMES],
			keyboard: [false; NUM_KEYS],
			display: [false; DISPLAY_SIZE],
		 }
	}
}