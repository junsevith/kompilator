use crate::intermediate::{CommandTranslator, TranslationError};
use crate::procedures::procedures::{FunctionRepository, ProcedureHandler};

pub struct MultiplicationProcedure;

impl MultiplicationProcedure {
    pub fn new() -> Self {
        MultiplicationProcedure
    }
}

impl ProcedureHandler for MultiplicationProcedure {
    fn initialize(
        &mut self,
        variable_stack: usize,
        instruction_start: usize,
        function_repository: &mut FunctionRepository,
    ) -> Result<(CommandTranslator, usize), TranslationError> {
        unimplemented!()
    }

    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut crate::variables::VariableDictionary,
        instructions: &mut CommandTranslator,
    ) -> Result<(), TranslationError> {
        unimplemented!()
    }
}