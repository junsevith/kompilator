use crate::intermediate::TranslationError::ErrorWithLocation;
use crate::intermediate::{Instruction, InstructionFactory, TranslationError};
use crate::preprocessor::Preprocessor;
use crate::procedures::assembly::AssemblyProcedure;
use crate::procedures::division::{division_procedure, DIVISION};
use crate::procedures::multiplication::{multiplication_procedure, MULTIPLICATION};
use crate::procedures::regular::RegularProcedure;
use crate::procedures::{FunctionRepository, ProcedureHandler};
use crate::structure::Program;
use crate::variables::VariableDictionary;
use std::collections::HashMap;

pub struct Translator {
    pub(crate) program: InstructionFactory,
    functions: FunctionRepository,
    memory_used: usize,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            program: InstructionFactory::new("alloc".to_string(), 0),
            memory_used: 10,
            functions: HashMap::new(),
        }
    }

    pub fn compile(&mut self, mut program: Program) -> Option<String> {
        match self.translate(program) {
            Ok(_) => {
                println!("Compilation successful!");
            }
            Err(error) => {
                println!("Error happened during compilation:");
                println!("{:?}", error);
                return None;
            }
        }
        Some(self.to_code(false))
    }

    pub fn translate(&mut self, mut program: Program) -> Result<(), TranslationError> {
        let literals = self.program.reserve_label("literals");

        self.program.push(Instruction::Goto(literals.clone()));

        let mut preprocessor = Preprocessor::new();
        preprocessor
            .process_program(&mut program)?;

        let defaults = vec![
            (MULTIPLICATION, Box::new(AssemblyProcedure::new(
                MULTIPLICATION,
                multiplication_procedure,
            ))),
            (DIVISION, Box::new(AssemblyProcedure::new(DIVISION, division_procedure)),)
        ];

        for (name, function) in defaults {
            self.prepare_procedure(&preprocessor.function_counter, &name.to_string(), function)?;
        }

        for procedure in program.procedures {
            let name = procedure.name.clone();

            let function: Box<dyn ProcedureHandler> = Box::new(RegularProcedure::new(procedure));

            self.prepare_procedure(&preprocessor.function_counter, &name, function)?;
        }

        let mut variables = VariableDictionary::new(self.memory_used);
        let mut intermediate =
            InstructionFactory::new("Main".to_string(), self.program.instructions.len());
        let main = self.program.reserve_label("main");
        intermediate.set_label(main.clone());

        for declaration in program.declarations {
            variables
                .add(declaration)?;
        }

        match intermediate.translate_commands(program.commands, &mut variables, &mut self.functions) {
            Ok(ok) => ok,
            Err(error) => {
                return Err(ErrorWithLocation(format!("{:?}", error), intermediate.action_stack))
            }
        };

        self.program.merge(intermediate);

        self.program.push(Instruction::Halt);

        self.process_code(&mut variables, main, literals)?;
        Ok(())
    }

    fn prepare_procedure(&mut self, function_counter: &HashMap<String, usize>, name: &String, mut function: Box<dyn ProcedureHandler>) -> Result<(), TranslationError> {
        match function_counter.get(name) {
            None | Some(0) => {
                //Do nothing
            }
            Some(1) => {
                self.functions.insert(name.clone(), function);
            }
            _ => {
                let (instructions, stack) = function.initialize(
                    self.memory_used,
                    self.program.instructions.len(),
                    &mut self.functions,
                )?;
                self.program.merge(instructions);
                self.memory_used = stack;
                self.functions.insert(name.clone(), function);
            }
        }
        Ok(())
    }
}
