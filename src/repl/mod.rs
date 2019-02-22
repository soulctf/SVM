use crate::svm::VM;
use std::io;
use std::io::Write;

pub struct REPL {
	vm: VM,
	program: Vec<String>,
}

impl REPL {
	pub fn new() -> REPL {
		REPL {
			vm: VM::new(),
			program: vec![]
		}
	}

	pub fn run(&mut self) { 

		loop {
			
			let mut buffer = String::new();

			let stdin = io::stdin();

			print!(">>> ");
       		io::stdout().flush().expect("Unable to flush stdout");

       		stdin.read_line(&mut buffer).expect("Unable to read line!");
       		let buffer = buffer.trim();

			match buffer {
				".registers" => {
					for i in 0..self.vm.registers.len() {
						println!("Register {}:{}", i, self.vm.registers[i]);
					}
				}, 
				".exit" => {
					println!("Goodbye!");
					break;
				},
				_ => {

				}
			}
		}
	}
}