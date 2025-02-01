use crate::intermediate::{CommandTranslator, Instruction, InstructionLine, TranslationError};
use crate::preprocessor::Preprocessor;
use crate::procedures::procedures::{
    new_function_repository, FunctionRepository, ProcedureHandler, RegularProcedure,
};
use crate::structure::{Declaration, Identifier, Procedure, Program, Value};
use crate::variables::{Pointer, Type, VariableDictionary};
use std::collections::HashMap;

pub struct Translator {
    pub(crate) program: CommandTranslator,
    functions: FunctionRepository,
    memory_used: usize,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            program: CommandTranslator::new("alloc".to_string(), 0),
            memory_used: 10,
            functions: new_function_repository(),
        }
    }

    pub fn translate(&mut self, mut program: Program) -> Result<(), TranslationError> {
        let literals = self.program.reserve_label("literals");

        self.program.push(Instruction::Goto(literals.clone()));

        let mut preprocessor = Preprocessor::new();
        preprocessor
            .process_program(&mut program)
            .map_err(|e| TranslationError::PreprocessorError(e))?;

        for procedure in program.procedures {
            self.translate_procedure(procedure, preprocessor.function_counter.clone())?;
        }

        let mut variables = VariableDictionary::new(self.memory_used);
        let mut intermediate =
            CommandTranslator::new("main".to_string(), self.program.instructions.len());
        let main = self.program.reserve_label("main");
        intermediate.set_label(main.clone());

        for declaration in program.declarations {
            variables
                .add(declaration)
                .map_err(|e| TranslationError::VariableError(e))?;
        }

        intermediate.translate_commands(program.commands, &mut variables, &mut self.functions)?;

        self.program.merge(intermediate);

        self.program.push(Instruction::Halt);

        self.process_code(&mut variables, main, literals)?;
        Ok(())
    }

    fn translate_procedure(
        &mut self,
        procedure: Procedure,
        function_counter: HashMap<String, usize>,
    ) -> Result<(), TranslationError> {
        let name = procedure.name.clone();

        let mut function = RegularProcedure::new(procedure);

        match function_counter.get(&name) {
            None | Some(0) => {
                //Do nothing
            }
            Some(1) => {
                self.functions.insert(name.clone(), Box::new(function));
            }
            _ => {
                let (instructions, stack) = function.initialize(
                    self.memory_used,
                    self.program.instructions.len(),
                    &mut self.functions,
                )?;
                self.program.merge(instructions);
                self.memory_used = stack;
                self.functions.insert(name.clone(), Box::new(function));
            }
        }
        Ok(())
    }

}
