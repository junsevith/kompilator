use crate::intermediate::program_translator::Translator;
use crate::intermediate::{Instruction, InstructionLine};
use crate::variables::Pointer;
use std::fmt::format;

impl Translator {
    pub fn to_code(&self) -> String {
        let mut code = String::new();
        for InstructionLine {
            instruction,
            comment,
            labels,
        } in &self.program.instructions
        {
            let instr = match instruction {
                Instruction::Get(Pointer::Cell(cell)) => format!("GET {}", cell),
                Instruction::Put(Pointer::Cell(cell)) => format!("PUT {}", cell),
                Instruction::Load(Pointer::Cell(cell)) => format!("LOAD {}", cell),
                Instruction::Store(Pointer::Cell(cell)) => format!("STORE {}", cell),
                Instruction::Load(Pointer::IndirectCell(cell)) => format!("LOADI {}", cell),
                Instruction::Store(Pointer::IndirectCell(cell)) => format!("STOREI {}", cell),
                Instruction::Add(Pointer::Cell(cell)) => format!("ADD {}", cell),
                Instruction::Subtr(Pointer::Cell(cell)) => format!("SUB {}", cell),
                Instruction::Add(Pointer::IndirectCell(cell)) => format!("ADDI {}", cell),
                Instruction::Subtr(Pointer::IndirectCell(cell)) => format!("SUBI {}", cell),
                Instruction::Set(num) => format!("SET {}", num),
                Instruction::Half => "HALF".to_string(),
                Instruction::Jump(num) => format!("JUMP {}", num),
                Instruction::Jpos(num) => format!("JPOS {}", num),
                Instruction::Jzero(num) => format!("JZERO {}", num),
                Instruction::Jneg(num) => format!("JNEG {}", num),
                Instruction::Return(Pointer::Cell(cell)) => format!("RTRN {}", cell),
                Instruction::Halt => "HALT".to_string(),
                _ => {
                    panic!("Instruction {:?} not allowed", instruction);
                }
            };
            code.push_str(format!("{}\n", instr).as_str());
        }
        code
    }
}
