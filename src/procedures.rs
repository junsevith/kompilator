use crate::intermediate::Instruction;
use crate::structure::Procedure;
use crate::variables::VariableDictionary;

pub trait ProcedureHandler {
    fn initialize(&mut self, stack: usize) -> (Vec<Instruction>, usize);
    fn call(&mut self, arguments: Vec<String>, stack: usize) -> Vec<Instruction>;
}

pub struct RegularProcedure {
    inline: bool,
    function: Option<Procedure>,
    variables: Option<VariableDictionary>,
}

impl RegularProcedure {
    pub fn new(procedure: Procedure) -> Self {
        RegularProcedure {
            inline: true,
            function: Some(procedure),
            variables: None,
        }
    }
}

impl ProcedureHandler for RegularProcedure {
    fn initialize(&mut self, stack: usize) -> (Vec<Instruction>, usize) {
        todo!()
    }

    fn call(&mut self, arguments: Vec<String>, stack: usize) -> Vec<Instruction> {
        todo!()
    }
}