use crate::procedures::procedures::ProcedureHandler;

pub mod procedures;
mod multiplication;
mod division;


pub struct DummyProcedure;

impl ProcedureHandler for DummyProcedure {
    fn initialize(
        &mut self,
        _variable_stack: usize,
        _instruction_start: usize,
        _function_repository: &mut procedures::FunctionRepository,
    ) -> Result<(crate::intermediate::CommandTranslator, usize), crate::intermediate::TranslationError> {
        panic!("This should never be called");
    }

    fn call(
        &mut self,
        _arguments: Vec<String>,
        _variable_dictionary: &mut crate::variables::VariableDictionary,
        _instructions: &mut crate::intermediate::CommandTranslator,
        _function_repository: &mut procedures::FunctionRepository,
    ) -> Result<(), crate::intermediate::TranslationError> {
        panic!("This should never be called");
    }
}