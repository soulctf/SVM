use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    pub remainder: u32,
    pub zero_flag: bool
}

impl VM {
	pub fn new() -> VM {
    	VM {
    		registers: [0; 32],
    		pc: 0,
    		program: vec![],
    		remainder: 0,
    		zero_flag: false,
    	}
	}

	pub fn run(&mut self) {
		loop {

			if self.pc >= self.program.len() {
            	break;
        	}

        	match self.decode_opcode() {
	            Opcode::HLT => {
	                println!("HLT encountered");
	                return;
	            },
	            Opcode::NOP => {
	            	println!("NOP");
	            },
	            Opcode::ADD => {
	            	let register1 = self.next_8_bits() as usize;
	            	let register2 = self.next_8_bits() as usize;
	            	let register3 = self.next_8_bits() as usize;
	            	self.registers[register3] = self.registers[register1] + self.registers[register2] as i32; 
	            },
				Opcode::SUB => {
					let register1 = self.next_8_bits() as usize;
	            	let register2 = self.next_8_bits() as usize;
	            	let register3 = self.next_8_bits() as usize;
	            	self.registers[register3] = self.registers[register1] - self.registers[register2] as i32; 
	            },
	            Opcode::MUL => {
	            	let register1 = self.registers[self.next_8_bits() as usize];
   		          	let register2 = self.registers[self.next_8_bits() as usize];
   		          	self.registers[self.next_8_bits() as usize] = register1 * register2;
	            },
	            Opcode::DIV => {
				    let register1 = self.registers[self.next_8_bits() as usize];
				    let register2 = self.registers[self.next_8_bits() as usize];
				    self.registers[self.next_8_bits() as usize] = register1 / register2;
				    self.remainder = (register1 % register2) as u32;
				},
	            Opcode::LOAD => {
	            	let register = self.next_8_bits() as usize;
	            	let value = self.next_16_bits();
	            	self.registers[register] = value as i32;
	            },
	            Opcode::JMPF => {
	            	let value = self.next_16_bits() as usize;
	            	self.pc += value;
	            },
	            Opcode::JMPB => {
	            	let value = self.next_16_bits() as usize;
	            	self.pc -= value;
	            },
	            Opcode::JMP => {
	            	let value = self.next_16_bits() as usize;
	            	self.pc = value;
	            },
	            Opcode::EQ => {
				    if self.registers[self.next_8_bits() as usize] == self.registers[self.next_8_bits() as usize] {
				    	self.zero_flag = true;
				    } else {				    	
				    	self.zero_flag = false;
				    }
	            },
	            Opcode::NEQ => {
				    if self.registers[self.next_8_bits() as usize] != self.registers[self.next_8_bits() as usize] {
				    	self.zero_flag = true;
				    } else {
				    	self.zero_flag = false;
				    }
	            },
	            Opcode::LT => {
					if self.registers[self.next_8_bits() as usize] < self.registers[self.next_8_bits() as usize] {
				    	self.zero_flag = true;
				    } else {
				    	self.zero_flag = false;
				    }
	            },
	            Opcode::GT => {
	            	if self.registers[self.next_8_bits() as usize] > self.registers[self.next_8_bits() as usize] {
				    	self.zero_flag = true;
				    } else {
				    	self.zero_flag = false;
				    }

	            },
	            Opcode::LTQ => {
					if self.registers[self.next_8_bits() as usize] <= self.registers[self.next_8_bits() as usize] {
				    	self.zero_flag = true;
				    } else {
				    	self.zero_flag = false;
				    }
	            },
	            Opcode::GTQ => {
	            	if self.registers[self.next_8_bits() as usize] >= self.registers[self.next_8_bits() as usize] {
				    	self.zero_flag = true;
				    } else {
				    	self.zero_flag = false;
				    }
	            },
	            Opcode::JE => {
	            	if self.zero_flag {
	            		self.pc += self.next_8_bits() as usize
	            	}
	            },
	            Opcode::JNE => {
	            	if !self.zero_flag {
	            		self.pc += self.next_8_bits() as usize
	            	}
	            },
	            _ => {
	              println!("Unrecognized opcode found! Terminating!");
	              return;
	            }
       		}
		}
	}

	fn decode_opcode(&mut self) -> Opcode {
		let opcode = Opcode::from(self.program[self.pc]);
  		self.pc += 1;
   		return opcode;
	}

	fn next_8_bits(&mut self) -> u8 {
		let result = self.program[self.pc];
		self.pc += 1;
		return result;
	}

	fn next_16_bits(&mut self) -> u16 {
		let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
		self.pc += 2;
		return result;
	}
}

#[cfg(test)]
mod tests {	
	use super::*;

	#[test]
	fn test() {

	}
}