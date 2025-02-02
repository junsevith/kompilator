
use crate::intermediate::{Instruction, InstructionFactory, TranslationError};
use crate::procedures::swap_vars::swap_commands;
use crate::procedures::{function_return, function_start, FunctionRepository, ProcedureHandler};
use crate::structure::{ArgumentDecl, Command, Declaration, Identifier, Procedure, Value};
use crate::variables::VariableDictionary;
use crate::variables::VariableError::VariableCollision;
use std::collections::HashMap;
use std::mem;
use crate::intermediate::TranslationError::{ErrorWithLocation, VariableError};

pub struct RegularProcedure {
    inline: bool,
    name: String,
    arguments: Vec<ArgumentDecl>,
    variables: Vec<Declaration>,
    commands: Vec<Command>,
    variable_dictionary: Option<VariableDictionary>,
}

impl ProcedureHandler for RegularProcedure {
    fn initialize(
        &mut self,
        variable_stack: usize,
        instruction_start: usize,
        mut function_repository: &mut FunctionRepository,
    ) -> Result<(InstructionFactory, usize), TranslationError> {
        self.inline = false;
        let mut dictionary = VariableDictionary::new(variable_stack);
        let mut translator = InstructionFactory::new(format!("Procedure {}", self.name), instruction_start);

        match self.construct_function(&mut function_repository, &mut dictionary, &mut translator) {
            Ok(ok) => ok,
            Err(error) => {
                return Err(ErrorWithLocation(format!("{:?}", error), translator.action_stack));
            }
        };

        let stack = dictionary.where_we_finished();

        self.variable_dictionary = Some(dictionary);

        Ok((translator, stack))
    }

    fn call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
        instructions: &mut InstructionFactory,
        function_repository: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        match self.inline {
            false => {
                self.prepare_for_call(arguments, variable_dictionary, instructions)?;
            }
            true => {
                self.inline_function(
                    arguments,
                    variable_dictionary,
                    instructions,
                    function_repository,
                )?;
            }
        }
        Ok(())
    }
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

    fn prepare_for_call(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
        instructions: &mut InstructionFactory,
    ) -> Result<(), TranslationError> {
        let self_dictionary = self.variable_dictionary.as_mut().unwrap();
        let ret = self_dictionary.write(Value::Identifier(Identifier::Variable(
            function_return(&self.name),
        )))?;

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

        instructions.push(Instruction::Goto(function_start(&self.name)));
        Ok(())
    }

    fn inline_function(
        &mut self,
        arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
        instructions: &mut InstructionFactory,
        function_repository: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        let mut variable_map = HashMap::new();

        instructions
            .action_stack
            .push(format!("Inlined procedure {}", self.name));

        for mut variable in self.variables.iter_mut() {
            let old_name;
            let new_name;
            match &mut variable {
                Declaration::ConstantDecl(name) | Declaration::VariableDecl(name) => {
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
            variable_map
                .insert(old_name.clone(), new_name)
                .map_or(Ok(()), |x| Err(VariableError(VariableCollision(x))))?;
            variable_dictionary.add(variable.clone())?;
        }

        // println!("{:?}", self.variables);

        for (declared, provided) in self.arguments.iter().zip(arguments.iter()) {
            match declared {
                ArgumentDecl::VariableArg(name) | ArgumentDecl::ArrayArg(name) => {
                    variable_map
                        .insert(name.clone(), provided.clone())
                        .map_or(Ok(()), |x| Err(VariableError(VariableCollision(x))))?;
                }
            }
        }

        swap_commands(&mut self.commands, &variable_map)?;

        instructions.translate_commands(
            mem::take(&mut self.commands),
            variable_dictionary,
            function_repository,
        )?;

        instructions.action_stack.pop();
        Ok(())
    }

    fn construct_function(&mut self, mut function_repository: &mut &mut FunctionRepository, mut dictionary: &mut VariableDictionary, translator: &mut InstructionFactory) -> Result<(), TranslationError> {
        for declaration in self.variables.iter() {
            dictionary.add(declaration.clone())?;
        }
        for argument in &self.arguments {
            dictionary.add_argument(argument.clone())?;
        }
        dictionary.add(Declaration::VariableDecl(function_return(&self.name)))?;

        translator.set_label(function_start(&self.name));
        translator.translate_commands(
            mem::take(&mut self.commands),
            &mut dictionary,
            &mut function_repository,
        )?;

        let ret = dictionary.write(Value::Identifier(Identifier::Variable(function_return(
            &self.name,
        ))))?;
        let ret = translator.prepare_pointer(ret, 2);
        translator.push(Instruction::Return(ret));
        Ok(())
    }
}
