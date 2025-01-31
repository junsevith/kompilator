use std::collections::HashMap;
use crate::intermediate::{Instruction, CommandTranslator};
use crate::preprocessor::Preprocessor;
use crate::procedures::{FunctionRepository, ProcedureHandler, RegularProcedure};
use crate::structure::{Procedure, Program};
use crate::variables::Pointer::Literal;
use crate::variables::{Pointer, VariableDictionary};

pub struct Translator {
    pub(crate) program: CommandTranslator,
    functions: FunctionRepository,
    memory_used: usize,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            program: CommandTranslator::new("".to_string(), 0),
            memory_used: 10,
            functions: FunctionRepository::new(),
        }
    }

    pub fn translate(&mut self, mut program: Program) {
        let literals = self.program.reserve_label("@literals");

        self.program.instructions.push((Instruction::Goto(literals.clone()), "alloc".to_string()));

        let mut preprocessor = Preprocessor::new();
        preprocessor.process_program(&mut program).expect("TODO: panic message");

        for procedure in program.procedures {
            self.translate_procedure(procedure, preprocessor.function_counter.clone());
        }

        let mut variables = VariableDictionary::new(self.memory_used);
        let mut intermediate = CommandTranslator::new("main".to_string(), self.program.instructions.len());
        let main = self.program.reserve_label("@main");
        intermediate.set_label(main.clone());

        for declaration in program.declarations {
            variables.add(declaration).expect("TODO: panic message");
        }
        
        intermediate.translate_commands(program.commands, &mut variables, &mut self.functions).expect("TODO: panic message");

        self.program.merge(intermediate);

        if self.program.literal_counter.is_empty() {
            self.program.instructions[0].0 = Instruction::Goto(main);
        } else {
            self.program.set_label(literals);

            self.allocate_literals();

            self.program.push(Instruction::Goto(main));

        }
    }

    fn translate_procedure(&mut self, procedure: Procedure, function_counter: HashMap<String, usize>) {

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
                let (instructions, stack) = function.initialize(self.memory_used, self.program.instructions.len(), &mut self.functions);
                self.program.merge(instructions);
                self.memory_used = stack;
                self.functions.insert(name.clone(), Box::new(function));
            }
        }


    }

    fn allocate_literals(&mut self) {

    }

}