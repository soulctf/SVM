struct VM {
	registers: [u32; 8],
	pc: usize,
	program: Vec<u8>;
}

impl VM {

	pub fn run(&mut self) {
		
		loop {
			if self.pc >= self.program.len() {
				break;
			}
			match self.serve_opcode() {
				OpCode::NOP => {
					return;
				},
				OpCode::HLT => {
					return;
				},
				_ => { 
					print!("You broke it :(");
					return;
				}
			}
		}

	}
}