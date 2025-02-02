use std::collections::HashMap;
use crate::intermediate::{InstructionFactory, TranslationError};
use crate::procedures::assembly::AssemblyProcedure;
use crate::procedures::division::{division_procedure, DIVISION};
use crate::procedures::multiplication::{multiplication_procedure, MULTIPLICATION};
use crate::variables::VariableDictionary;

pub mod division;
pub mod assembly;
pub mod regular;
mod swap_vars;
pub mod multiplication;

pub struct DummyProcedure;

pub type FunctionRepository = HashMap<String, Box<dyn ProcedureHandler>>;

pub const SHIFT_LEFT: &str = "@shift_left";
pub const SHIFT_RIGHT: &str = "@shift_right";

pub const FUNCTION_START: &str = "@start@";

pub const FUNCTION_RETURN: &str = "@return@";

pub trait ProcedureHandler {
    fn initialize(
        &mut self,
        variable_stack: usize,
        instruction_start: usize,
        function_repository: &mut FunctionRepository,
    ) -> Result<(InstructionFactory, usize), TranslationError>;
    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
        instructions: &mut InstructionFactory,
        function_repository: &mut FunctionRepository,
    ) -> Result<(), TranslationError>;
}

pub fn function_start(name: &str) -> String {
    format!("{}{}", FUNCTION_START, name)
}

pub fn function_return(name: &str) -> String {
    format!("{}{}", FUNCTION_RETURN, name)
}

pub fn new_function_repository() -> FunctionRepository {
    let mut new = FunctionRepository::new();
    new.insert(
        MULTIPLICATION.to_string(),
        Box::new(AssemblyProcedure::new(
            MULTIPLICATION,
            multiplication_procedure,
        )),
    );
    new.insert(
        DIVISION.to_string(),
        Box::new(AssemblyProcedure::new(DIVISION, division_procedure)),
    );
    new
}

impl ProcedureHandler for DummyProcedure {
    fn initialize(
        &mut self,
        _variable_stack: usize,
        _instruction_start: usize,
        _function_repository: &mut FunctionRepository,
    ) -> Result<
        (InstructionFactory, usize),
        TranslationError,
    > {
        panic!("This should never be called");
    }

    fn call(
        &mut self,
        _arguments: Vec<String>,
        _variable_dictionary: &mut VariableDictionary,
        _instructions: &mut InstructionFactory,
        _function_repository: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        panic!("This should never be called");
    }
}
