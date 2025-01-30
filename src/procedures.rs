use crate::intermediate::Instruction;
use crate::structure::{ArgumentDecl, Command, Declaration, Procedure};
use crate::variables::VariableDictionary;
use std::collections::HashMap;
pub type FunctionRepository = HashMap<String, Box<dyn ProcedureHandler>>;

pub trait ProcedureHandler {
    fn initialize(&mut self, stack: usize) -> (Vec<Instruction>, usize);
    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
    ) -> Vec<Instruction>;
}

pub struct RegularProcedure {
    inline: bool,
    name: String,
    arguments: Vec<ArgumentDecl>,
    variables: Vec<Declaration>,
    commands: Vec<Command>,
    variable_dictionary: Option<VariableDictionary>,
}

impl RegularProcedure {
    pub fn new(procedure: Procedure) -> Self {
        let Procedure {
            name,
            arguments,
            declarations,
            commands,
        } = procedure;

        RegularProcedure {
            inline: true,
            name,
            arguments,
            variables: declarations,
            commands,
            variable_dictionary: None,
        }
    }
}

impl ProcedureHandler for RegularProcedure {
    fn initialize(&mut self, stack: usize) -> (Vec<Instruction>, usize) {
        todo!()
    }

    fn call(&mut self, arguments: Vec<String>, variable_dictionary: &mut VariableDictionary) -> Vec<Instruction> {
        todo!()
    }
}
