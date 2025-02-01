use crate::intermediate::{CommandTranslator, Instruction, TranslationError};
use crate::procedures::division::DivisionProcedure;
use crate::procedures::multiplication::MultiplicationProcedure;
use crate::structure::{ArgumentDecl, Command, Declaration, Procedure};
use crate::variables::VariableDictionary;
use std::collections::HashMap;
use std::mem;

pub type FunctionRepository = HashMap<String, Box<dyn ProcedureHandler>>;

pub fn new_function_repository() -> FunctionRepository {
    let mut new = FunctionRepository::new();
    new.insert(
        "@multiplication".to_string(),
        Box::new(MultiplicationProcedure),
    );
    new.insert("@division".to_string(), Box::new(DivisionProcedure));
    new
}

pub trait ProcedureHandler {
    fn initialize(
        &mut self,
        variable_stack: usize,
        instruction_start: usize,
        function_repository: &mut FunctionRepository,
    ) -> Result<(CommandTranslator, usize), TranslationError>;
    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
    ) -> Result<CommandTranslator, TranslationError>;
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
    fn initialize(
        &mut self,
        variable_stack: usize,
        instruction_start: usize,
        mut function_repository: &mut FunctionRepository,
    ) -> Result<(CommandTranslator, usize), TranslationError> {
        self.inline = false;
        let mut dictionary = VariableDictionary::new(variable_stack);
        for declaration in self.variables.iter() {
            dictionary
                .add(declaration.clone())
                .map_err(|e| TranslationError::VariableError(e))?;
        }
        for argument in &self.arguments {
            dictionary
                .add_argument(argument.clone())
                .map_err(|e| TranslationError::VariableError(e))?;
        }
        dictionary
            .add(Declaration::VariableDecl(format!("@{}_return", self.name)))
            .map_err(|e| TranslationError::VariableError(e))?;

        let mut translator = CommandTranslator::new(self.name.clone(), instruction_start);
        translator
            .translate_commands(
                mem::take(&mut self.commands),
                &mut dictionary,
                &mut function_repository,
            )?;

        let stack = dictionary.where_we_finished();

        self.variable_dictionary = Some(dictionary);

        Ok((translator, stack))
    }

    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
    ) -> Result<CommandTranslator, TranslationError> {
        let mut instructions = CommandTranslator::new(format!("{}_call", self.name), 0);
        match self.inline {
            false => {
                instructions.push(Instruction::LoadCurrentLocation);

            }
            true => {}
        }
        Ok(instructions)
    }
}
