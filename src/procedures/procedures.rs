use crate::intermediate::{CommandTranslator, Instruction, TranslationError};
use crate::procedures::division::DivisionProcedure;
use crate::procedures::multiplication::MultiplicationProcedure;
use crate::structure::{ArgumentDecl, Command, Declaration, Identifier, Procedure, Value};
use crate::variables::VariableDictionary;
use std::collections::HashMap;
use std::mem;

pub type FunctionRepository = HashMap<String, Box<dyn ProcedureHandler>>;

pub fn new_function_repository() -> FunctionRepository {
    let mut new = FunctionRepository::new();
    new.insert(
        "@multiply".to_string(),
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
        instructions: &mut CommandTranslator,
    ) -> Result<(), TranslationError>;
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
        translator.set_label(format!("{}_start", self.name));
        translator
            .translate_commands(
                mem::take(&mut self.commands),
                &mut dictionary,
                &mut function_repository,
            )?;

        let ret = dictionary.write(Value::Identifier(Identifier::Variable(format!("@{}_return", self.name)))).map_err(|e| TranslationError::VariableError(e))?;
        let ret = translator.prepare_pointer(ret, 2);
        translator.push(Instruction::Return(ret));

        let stack = dictionary.where_we_finished();

        self.variable_dictionary = Some(dictionary);

        Ok((translator, stack))
    }

    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
        instructions: &mut CommandTranslator,
    ) -> Result<(), TranslationError> {
        instructions.action_stack.push(format!("Call {}", self.name));
        match self.inline {
            false => {
                let self_dictionary = self.variable_dictionary.as_mut().unwrap();
                let ret = self_dictionary.write(Value::Identifier(Identifier::Variable(format!("@{}_return", self.name)))).map_err(|e| TranslationError::VariableError(e))?;
                let ret = instructions.prepare_pointer(ret, 2);



                for (provided, declared) in arguments.iter().zip(self.arguments.iter()) {
                    match declared {
                        ArgumentDecl::VariableArg(name) => {
                            instructions.action_stack.push(format!("var_{} -> arg_{}", provided, name));
                            let value = variable_dictionary.write(Value::Identifier(Identifier::Variable(provided.clone()))).map_err(|e| TranslationError::VariableError(e))?;
                            let value = instructions.prepare_pointer(value, 2);

                            let place = self_dictionary.write(Value::Identifier(Identifier::Variable(name.clone()))).map_err(|e| TranslationError::VariableError(e))?;
                            let place = instructions.prepare_pointer(place, 3);

                            instructions.push(Instruction::Load(value.location()));
                            instructions.push(Instruction::Store(place.location()));
                            instructions.action_stack.pop();
                        }
                        ArgumentDecl::ArrayArg(name) => {}
                    }
                }

                instructions.action_stack.push("set return".to_string());
                instructions.push(Instruction::LoadCurrentLocation);
                instructions.push(Instruction::Store(ret));
                instructions.action_stack.pop();

                instructions.push(Instruction::Goto(format!("{}_start", self.name)));

            }
            true => {}
        }
        Ok(())
    }
}
