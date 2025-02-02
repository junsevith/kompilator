use crate::intermediate::{Instruction, InstructionFactory, TranslationError};
use crate::procedures::FunctionRepository;
use crate::procedures::{function_return, function_start, ProcedureHandler};
use crate::structure::{Declaration, Identifier, Value};
use crate::variables::VariableDictionary;

pub struct AssemblyProcedure {
    name: &'static str,
    inline: bool,
    variable_dictionary: Option<VariableDictionary>,
    procedure_factory: fn(&mut InstructionFactory),
}

impl AssemblyProcedure {
    pub fn new(name: &'static str, procedure: fn(&mut InstructionFactory)) -> Self {
        AssemblyProcedure {
            name,
            inline: true,
            variable_dictionary: None,
            procedure_factory: procedure,
        }
    }
}

impl ProcedureHandler for AssemblyProcedure {
    fn initialize(
        &mut self,
        variable_stack: usize,
        instruction_start: usize,
        _function_repository: &mut FunctionRepository,
    ) -> Result<(InstructionFactory, usize), TranslationError> {
        self.inline = false;
        let mut dictionary = VariableDictionary::new(variable_stack);
        dictionary.add(Declaration::VariableDecl(function_return(self.name)))?;

        let mut instructions = InstructionFactory::new(self.name.to_string(), instruction_start);
        instructions.set_label(function_start(self.name));

        (self.procedure_factory)(&mut instructions);

        let ret = dictionary.write(Value::Identifier(Identifier::Variable(function_return(
            self.name
        ))))?;
        let ret = instructions.prepare_pointer(ret, 2);
        instructions.push(Instruction::Return(ret));

        let stack = dictionary.where_we_finished();

        self.variable_dictionary = Some(dictionary);

        Ok((instructions, stack))
    }

    fn call(
        &mut self,
        _arguments: Vec<String>,
        variable_dictionary: &mut VariableDictionary,
        instructions: &mut InstructionFactory,
        _function_repository: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        match self.inline {
            true => {
                instructions
                    .action_stack
                    .push(format!("Inlined {}", self.name));
                (self.procedure_factory)(instructions);
                instructions.action_stack.pop();
                Ok(())
            }
            false => {
                let ret = self.variable_dictionary.as_mut().unwrap().write(Value::Identifier(Identifier::Variable(
                    function_return(self.name),
                )))?;
                let ret = instructions.prepare_pointer(ret, 9);

                instructions.action_stack.push("set return".to_string());
                instructions.push(Instruction::LoadKPlus3);
                instructions.push(Instruction::Store(ret));
                instructions.action_stack.pop();

                instructions.push(Instruction::Goto(function_start(&self.name)));
                Ok(())
            }
        }
    }
}
