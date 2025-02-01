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
    ) -> (crate::intermediate::CommandTranslator, usize) {
        unimplemented!()
    }

    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut crate::variables::VariableDictionary,
    ) -> crate::intermediate::CommandTranslator {
        unimplemented!()
    }
}