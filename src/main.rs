pub mod svm;
pub mod instruction;
pub mod repl;

fn main() {
	let mut repl = repl::REPL::new();
	repl.run();
}