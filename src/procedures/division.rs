use crate::intermediate::{Instruction, InstructionFactory};
use crate::variables::Pointer;

pub const DIVISION: &str = "@division";

pub fn division_procedure(instr: &mut InstructionFactory) {
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Store(Pointer::Cell(4)));

    instr.push(Instruction::Load(Pointer::Cell(6)));
    instr.push(Instruction::Jzero(64));
    instr.push(Instruction::Jpos(3));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(6)));
    instr.push(Instruction::Store(Pointer::Cell(2)));

    instr.push(Instruction::Load(Pointer::Cell(7)));
    instr.push(Instruction::Jzero(2));
    instr.push(Instruction::Jump(4));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Store(Pointer::Cell(2)));
    instr.push(Instruction::Jump(54));
    instr.push(Instruction::Jpos(3));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(7)));
    instr.push(Instruction::Store(Pointer::Cell(3)));

    instr.push(Instruction::Subtr(Pointer::Cell(2)));
    instr.push(Instruction::Jpos(29));

    instr.push(Instruction::Load(Pointer::Literal(1)));
    instr.push(Instruction::Store(Pointer::Cell(5)));

    instr.push(Instruction::Load(Pointer::Cell(3)));
    instr.push(Instruction::Add(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(2)));
    instr.push(Instruction::Jpos(7));
    instr.push(Instruction::Add(Pointer::Cell(2)));
    instr.push(Instruction::Store(Pointer::Cell(3)));
    instr.push(Instruction::Load(Pointer::Cell(5)));
    instr.push(Instruction::Add(Pointer::Cell(5)));
    instr.push(Instruction::Store(Pointer::Cell(5)));
    instr.push(Instruction::Jump(-9));

    instr.push(Instruction::Load(Pointer::Cell(2)));
    instr.push(Instruction::Jzero(15));
    instr.push(Instruction::Subtr(Pointer::Cell(3)));
    instr.push(Instruction::Jneg(5));
    instr.push(Instruction::Store(Pointer::Cell(2)));
    instr.push(Instruction::Load(Pointer::Cell(4)));
    instr.push(Instruction::Add(Pointer::Cell(5)));
    instr.push(Instruction::Store(Pointer::Cell(4)));
    instr.push(Instruction::Load(Pointer::Cell(5)));
    instr.push(Instruction::Half);
    instr.push(Instruction::Jzero(6));
    instr.push(Instruction::Store(Pointer::Cell(5)));
    instr.push(Instruction::Load(Pointer::Cell(3)));
    instr.push(Instruction::Half);
    instr.push(Instruction::Store(Pointer::Cell(3)));
    instr.push(Instruction::Jump(-16));

    instr.push(Instruction::Load(Pointer::Cell(6)));
    instr.push(Instruction::Jpos(7));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(2)));
    instr.push(Instruction::Store(Pointer::Cell(2)));
    instr.push(Instruction::Load(Pointer::Cell(7)));
    instr.push(Instruction::Jpos(4));
    instr.push(Instruction::Jump(12));
    instr.push(Instruction::Load(Pointer::Cell(7)));
    instr.push(Instruction::Jpos(10));
    instr.push(Instruction::Load(Pointer::Literal(1)));
    instr.push(Instruction::Add(Pointer::Cell(4)));
    instr.push(Instruction::Store(Pointer::Cell(4)));
    instr.push(Instruction::Subtr(Pointer::Cell(0)));
    instr.push(Instruction::Subtr(Pointer::Cell(4)));
    instr.push(Instruction::Store(Pointer::Cell(4)));
    instr.push(Instruction::Load(Pointer::Cell(2)));
    instr.push(Instruction::Add(Pointer::Cell(7)));
    instr.push(Instruction::Store(Pointer::Cell(2)));

}
