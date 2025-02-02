use crate::intermediate::{Instruction, InstructionFactory};
use crate::variables::Pointer;

pub const MULTIPLICATION: &str = "@multiplication";

pub fn multiplication_procedure(instr: &mut InstructionFactory) {
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Store(Pointer::Cell(4)));

    instr.push(Instruction::Load(Pointer::Cell(6)));
    instr.push(Instruction::Jzero(37));
    instr.push(Instruction::Jpos(3));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(6)));
    instr.push(Instruction::Store(Pointer::Cell(2)));

    instr.push(Instruction::Load(Pointer::Cell(7)));
    instr.push(Instruction::Jzero(31));
    instr.push(Instruction::Jpos(3));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(7)));
    instr.push(Instruction::Store(Pointer::Cell(3)));

    instr.push(Instruction::Load(Pointer::Cell(3)));
    instr.push(Instruction::Half);
    instr.push(Instruction::Add(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(3)));
    instr.push(Instruction::Jzero(4));
    instr.push(Instruction::Load(Pointer::Cell(4)));
    instr.push(Instruction::Add(Pointer::Cell(2)));
    instr.push(Instruction::Store(Pointer::Cell(4)));
    instr.push(Instruction::Load(Pointer::Cell(3)));
    instr.push(Instruction::Half);
    instr.push(Instruction::Jzero(6));
    instr.push(Instruction::Store(Pointer::Cell(3)));
    instr.push(Instruction::Load(Pointer::Cell(2)));
    instr.push(Instruction::Add(Pointer::Cell(2)));
    instr.push(Instruction::Store(Pointer::Cell(2)));
    instr.push(Instruction::Jump(-15));

    instr.push(Instruction::Load(Pointer::Cell(6)));
    instr.push(Instruction::Jneg(4));
    instr.push(Instruction::Load(Pointer::Cell(7)));
    instr.push(Instruction::Jneg(4));
    instr.push(Instruction::Jump(6));
    instr.push(Instruction::Load(Pointer::Cell(7)));
    instr.push(Instruction::Jneg(4));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(4)));
    instr.push(Instruction::Store(Pointer::Cell(4)));

}
