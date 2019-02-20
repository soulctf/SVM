pub mod svm;
pub mod instruction;

fn main() {
	let mut program: Vec<u8> = Vec::new(); 
	program.push(17); 	
	program.push(1);
	program.push(0);
	program.push(20);
	program.push(17); 	
	program.push(2);
	program.push(0);
	program.push(50);
	program.push(17); 	
	program.push(3);
	program.push(0);
	program.push(60);
	program.push(20);
	program.push(1);
	program.push(2);	
	program.push(4);
	program.push(32);
	program.push(0);
	program.push(4);
	program.push(20);
	program.push(1);
	program.push(3);	
	program.push(4);
	program.push(0);

	let mut vm = svm::VM::new(program);
	vm.run();
	println!("{} {}", vm.registers[4], vm.remainder);
}