use crate::intermediate::TranslationError::VariableError;
use crate::intermediate::{CommandTranslator, Instruction, TranslationError};
use crate::procedures::division::DivisionProcedure;
use crate::procedures::multiplication::MultiplicationProcedure;
use crate::structure::{ArgumentDecl, Command, Condition, Declaration, Identifier, Operation, Procedure, Value};
use crate::variables::VariableDictionary;
use crate::variables::VariableError::{NoArray, NoVariable, VariableCollision};
use std::collections::HashMap;
use std::mem;

pub type FunctionRepository = HashMap<String, Box<dyn ProcedureHandler>>;

pub fn new_function_repository() -> FunctionRepository {
    let mut new = FunctionRepository::new();
    new.insert("@multiply".to_string(), Box::new(MultiplicationProcedure));
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
        function_repository: &mut FunctionRepository,
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

    fn swap_commands(commands: &mut Vec<Command>, variable_map: &HashMap<String, String>) -> Result<(), TranslationError> {
        for command in commands {
            match command {
                Command::Assign(identifier, Operation{ operator, left, right }) => {
                    Self::swap_identifier(identifier, variable_map)?;
                    Self::swap_values(left, variable_map)?;
                    Self::swap_values(right, variable_map)?;
                }
                Command::If(Condition{ operator, left, right }, commands) => {
                    Self::swap_values(left, variable_map)?;
                    Self::swap_values(right, variable_map)?;
                    Self::swap_commands(commands, variable_map)?;
                }
                Command::IfElse(Condition{ operator, left, right }, commands1, commands2) => {
                    Self::swap_values(left, variable_map)?;
                    Self::swap_values(right, variable_map)?;
                    Self::swap_commands(commands1, variable_map)?;
                    Self::swap_commands(commands2, variable_map)?;
                }
                Command::While(Condition{ operator, left, right }, commands) => {
                    Self::swap_values(left, variable_map)?;
                    Self::swap_values(right, variable_map)?;
                    Self::swap_commands(commands, variable_map)?;
                }
                Command::Repeat(Condition{ operator, left, right }, commands) => {
                    Self::swap_values(left, variable_map)?;
                    Self::swap_values(right, variable_map)?;
                    Self::swap_commands(commands, variable_map)?;
                }
                Command::For(iter, first, second, commands) => {
                    *iter = variable_map.get(iter).unwrap().clone();
                    Self::swap_values(first, variable_map)?;
                    Self::swap_values(second, variable_map)?;
                    Self::swap_commands(commands, variable_map)?;
                }
                Command::ForDown(iter, first, second, commands) => {
                    *iter = variable_map.get(iter).unwrap().clone();
                    Self::swap_values(first, variable_map)?;
                    Self::swap_values(second, variable_map)?;
                    Self::swap_commands(commands, variable_map)?;
                }
                Command::FunctionCall(name, arguments) => {
                    for argument in arguments {
                        *argument = variable_map.get(argument).unwrap().clone();
                    }
                }
                Command::Read(identifier) => {
                    Self::swap_identifier(identifier, variable_map)?;
                }
                Command::Write(value) => {
                    Self::swap_values(value, variable_map)?;
                }
            }
        }
        Ok(())
    }

    fn swap_values(value: &mut Value, variable_map: &HashMap<String, String>) -> Result<(), TranslationError> {
        match value {
            Value::Literal(_) => {}
            Value::Identifier(identifier) => {
                Self::swap_identifier(identifier, variable_map)?;
            }
        }
        Ok(())
    }

    fn swap_identifier(identifier: &mut Identifier, variable_map: &HashMap<String, String>) -> Result<(), TranslationError> {
        match identifier {
            Identifier::Variable(name) => {
                let new_name = variable_map.get(name).map_or(Err(VariableError(NoVariable(name.clone()))), |x| Ok(x))?;
                *name = new_name.clone();
            }
            Identifier::ArrayLit(name, _) => {
                let new_name = variable_map.get(name).map_or(Err(VariableError(NoArray(name.clone()))), |x| Ok(x))?;
                *name = new_name.clone();
            }
            Identifier::ArrayVar(name, variable) => {
                let new_name =  variable_map.get(name).map_or(Err(VariableError(NoArray(name.clone()))), |x| Ok(x))?;
                *name = new_name.clone();
                let new_variable = variable_map.get(variable).map_or(Err(VariableError(NoVariable(variable.clone()))), |x| Ok(x))?;
                *variable = new_variable.clone();
            }
        }
        Ok(())
    }

    fn prepare_arguments(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
        instructions: &mut CommandTranslator,
    ) -> Result<(), TranslationError> {
        let self_dictionary = self.variable_dictionary.as_mut().unwrap();
        let ret = self_dictionary.write(Value::Identifier(Identifier::Variable(format!(
            "@{}@return",
            self.name
        ))))?;

        let ret = instructions.prepare_pointer(ret, 2);

        for (provided, declared) in arguments.iter().zip(self.arguments.iter()) {
            match declared {
                ArgumentDecl::VariableArg(name) => {
                    instructions
                        .action_stack
                        .push(format!("var_{} -> arg_{}", provided, name));

                    let value = variable_dictionary
                        .write(Value::Identifier(Identifier::Variable(provided.clone())))?;

                    let value = instructions.prepare_pointer(value, 2);

                    let place = self_dictionary
                        .write(Value::Identifier(Identifier::Variable(name.clone())))?;

                    let place = instructions.prepare_pointer(place, 3);

                    instructions.push(Instruction::Load(value.location()));
                    instructions.push(Instruction::Store(place.location()));
                    instructions.action_stack.pop();
                }
                ArgumentDecl::ArrayArg(name) => {
                    instructions
                        .action_stack
                        .push(format!("arr_{} -> arr_arg_{}", provided, name));

                    let value = variable_dictionary.get_array_offset(&provided)?;

                    let place = self_dictionary.get_array_offset(&name)?;

                    instructions.push(Instruction::Load(value));
                    instructions.push(Instruction::Store(place));
                    instructions.action_stack.pop();
                }
            }
        }

        instructions.action_stack.push("set return".to_string());
        instructions.push(Instruction::LoadKPlus3);
        instructions.push(Instruction::Store(ret));
        instructions.action_stack.pop();

        instructions.push(Instruction::Goto(format!("{}_start", self.name)));
        Ok(())
    }

    fn inline_function(&mut self, arguments: Vec<String>, variable_dictionary: &mut VariableDictionary, instructions: &mut CommandTranslator, function_repository: &mut FunctionRepository) -> Result<(), TranslationError> {
        let mut variable_map = HashMap::new();

        for mut variable in self.variables.iter_mut() {
            let old_name;
            let new_name;
            match &mut variable {
                Declaration::ConstantDecl(name) |
                Declaration::VariableDecl(name) => {
                    old_name = name.clone();
                    new_name = format!("@{}@{}", self.name, name);
                    *name = new_name.clone();
                }
                Declaration::ArrayDecl(name, _, _) => {
                    old_name = name.clone();
                    new_name = format!("@{}@{}", self.name, name);
                    *name = new_name.clone();
                }
            }
            variable_map.insert(old_name.clone(), new_name).map_or(Ok(()), |x| Err(VariableError(VariableCollision(x))))?;
            variable_dictionary.add(variable.clone())?;
        }

        // println!("{:?}", self.variables);

        for (declared, provided) in self.arguments.iter().zip(arguments.iter()) {
            match declared {
                ArgumentDecl::VariableArg(name) | ArgumentDecl::ArrayArg(name) => {
                    variable_map.insert(name.clone(), provided.clone()).map_or(Ok(()), |x| Err(VariableError(VariableCollision(x))))?;
                }
            }
        }

        Self::swap_commands(&mut self.commands, &variable_map)?;

        // println!("{:?}", self.commands);

        instructions.action_stack.push(format!("Inlined {}", self.name));

        instructions.translate_commands(
            mem::take(&mut self.commands),
            variable_dictionary,
            function_repository,
        )?;

        instructions.action_stack.pop();
        Ok(())
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
            dictionary.add(declaration.clone())?;
        }
        for argument in &self.arguments {
            dictionary.add_argument(argument.clone())?;
        }
        dictionary.add(Declaration::VariableDecl(format!("@{}@return", self.name)))?;

        let mut translator = CommandTranslator::new(self.name.clone(), instruction_start);
        translator.set_label(format!("{}_start", self.name));
        translator.translate_commands(
            mem::take(&mut self.commands),
            &mut dictionary,
            &mut function_repository,
        )?;

        let ret = dictionary.write(Value::Identifier(Identifier::Variable(format!(
            "@{}@return",
            self.name
        ))))?;
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
        function_repository: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        instructions
            .action_stack
            .push(format!("Call {}", self.name));

        match self.inline {
            false => {
                self.prepare_arguments(arguments, variable_dictionary, instructions)?;
            }
            true => {
                self.inline_function(arguments, variable_dictionary, instructions, function_repository)?;
            }
        }
        Ok(())
    }
}
