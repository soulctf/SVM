#[derive(Debug, PartialEq)]
pub enum Opcode {
	//00-0f General
	NOP,
	HLT,
	TERM,
	//10-1f Math
	LOAD, 
	ADD,
	SUB,
	MUL,
	DIV,
	//20-2f Functionality
	JMPF,
	JMPB,
	JMP,
	EQ,
	NEQ,
	LT,
	GT,
	LTQ,
	GTQ,
	JE,
	JNE
}

pub struct Instruction {
	opcode: Opcode
}

impl Instruction {
	pub fn new(opcode: Opcode) -> Instruction {
		Instruction {
			opcode: opcode
		}
	}
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::HLT,
            1 => return Opcode::NOP,
            17 => return Opcode::LOAD,
            18 => return Opcode::ADD,
            19 => return Opcode::SUB,
            20 => return Opcode::MUL,
            21 => return Opcode::DIV,
            32 => return Opcode::JMPF,
            33 => return Opcode::JMPB,
            34 => return Opcode::JMP,
            35 => return Opcode::EQ,
            36 => return Opcode::NEQ,
            37 => return Opcode::LT,
            38 => return Opcode::GT,
            39 => return Opcode::LTQ,
            40 => return Opcode::GTQ,
            41 => return Opcode::JE,
            42 => return Opcode::JNE,
            _ => return Opcode::TERM
        }
    }

}
