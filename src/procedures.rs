use crate::intermediate::CommandTranslator;
use crate::structure::{ArgumentDecl, Command, Declaration, Procedure};
use crate::variables::VariableDictionary;
use std::collections::HashMap;
use std::mem;

pub type FunctionRepository = HashMap<String, Box<dyn ProcedureHandler>>;

pub trait ProcedureHandler {
    fn initialize(
        &mut self,
        variable_stack: usize,
        instruction_start: usize,
        function_repository: &mut FunctionRepository,
    ) -> (CommandTranslator, usize);
    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
    ) -> CommandTranslator;
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
    ) -> (CommandTranslator, usize) {
        self.inline = false;
        let mut dictionary = VariableDictionary::new(variable_stack);
        for declaration in self.variables.iter() {
            dictionary
                .add(declaration.clone())
                .expect("TODO: panic message");
        }
        for argument in &self.arguments {
            dictionary
                .add_argument(argument.clone())
                .expect("TODO: panic message");
        }
        dictionary
            .add(Declaration::VariableDecl(format!("@{}_return", self.name)))
            .expect("TODO: panic message");

        let mut translator = CommandTranslator::new(self.name.clone(), instruction_start);
        translator
            .translate_commands(
                mem::take(&mut self.commands),
                &mut dictionary,
                &mut function_repository,
            )
            .expect("TODO: panic message");

        let stack = dictionary.where_we_finished();

        self.variable_dictionary = Some(dictionary);

        (translator, stack)
    }

    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
    ) -> CommandTranslator {
        todo!()
    }
}
