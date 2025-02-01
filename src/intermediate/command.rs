use crate::intermediate::{CommandTranslator, Instruction, TranslationError};
use crate::procedures::procedures::FunctionRepository;
use crate::structure::{Command, Identifier, Value};
use crate::variables::{Pointer, VariableDictionary};

impl CommandTranslator {
    pub(crate) fn translate_command(
        &mut self,
        command: Command,
        variables: &mut VariableDictionary,
        functions: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        match command {
            Command::Assign(variable, operation) => {
                self.action_stack.push("Assignment".to_string());
                self.translate_assign(variable, operation, variables, functions)?;
            }
            Command::If(condition, commands) => {
                self.action_stack.push("If".to_string());

                let end_label = self.reserve_label("if on false");
                self.handle_condition(condition, variables, &end_label)?;

                self.action_stack.push("true".to_string());
                self.translate_commands(commands, variables, functions)?;
                self.action_stack.pop();

                self.set_label(end_label);
            }
            Command::IfElse(condition, yes, no) => {
                self.action_stack.push("IfElse".to_string());

                let end_label = self.reserve_label("ifelse end");
                let else_label = self.reserve_label("ifelse else");

                self.handle_condition(condition, variables, &else_label)?;

                self.action_stack.push("true".to_string());
                self.translate_commands(yes, variables, functions)?;
                self.push(Instruction::Goto(end_label.clone()));
                self.action_stack.pop();

                self.set_label(else_label);
                self.action_stack.push("false".to_string());
                self.translate_commands(no, variables, functions)?;
                self.action_stack.pop();

                self.set_label(end_label);
            }
            Command::While(condition, commands) => {
                self.action_stack.push("While".to_string());

                let start_label = self.reserve_label("while start");
                let end_label = self.reserve_label("while end");
                self.set_label(start_label.clone());
                self.handle_condition(condition, variables, &end_label)?;
                self.translate_commands(commands, variables, functions)?;
                self.push(Instruction::Goto(start_label));
                self.set_label(end_label);
            }
            Command::Repeat(condition, commands) => {
                self.action_stack.push("Repeat".to_string());

                let start_label = self.reserve_label("repeat start");
                self.set_label(start_label.clone());
                self.translate_commands(commands, variables, functions)?;
                self.handle_condition(condition, variables, &start_label)?;

            }
            Command::For(iter, start, end, commands) => {
                self.action_stack.push("For".to_string());

                let iter_end_type = variables.write_(Value::Identifier(Identifier::Variable(format!("{}_end", iter))))?;
                let iter_end_ptr = self.prepare_pointer(iter_end_type, 2);
                let iter_type = variables.write_(Value::Identifier(Identifier::Variable(iter)))?;
                let iter_ptr = self.prepare_pointer(iter_type, 1);

                let start = variables.read_(start)?;
                self.load(start);
                self.push(Instruction::Store(iter_ptr));
                let end = variables.read_(end)?;
                self.load(end);
                self.push(Instruction::Store(iter_end_ptr));

                let start_label = self.reserve_label("for start");
                self.set_label(start_label.clone());
                self.translate_commands(commands, variables, functions)?;

                self.load(iter_type);
                self.push(Instruction::Add(Pointer::Literal(1)));
                self.push(Instruction::Store(iter_ptr));
                self.push(Instruction::Subtr(iter_end_ptr));
                self.push(Instruction::Jpos(2));
                self.push(Instruction::Goto(start_label));

            }
            Command::ForDown(iter, start, end, commands) => {
                self.action_stack.push("ForDown".to_string());

                let iter_end_type = variables.write_(Value::Identifier(Identifier::Variable(format!("{}_end", iter))))?;
                let iter_end_ptr = self.prepare_pointer(iter_end_type, 2);
                let iter_type = variables.write_(Value::Identifier(Identifier::Variable(iter)))?;
                let iter_ptr = self.prepare_pointer(iter_type, 1);

                let start = variables.read_(start)?;
                self.load(start);
                self.push(Instruction::Store(iter_ptr));
                let end = variables.read_(end)?;
                self.load(end);
                self.push(Instruction::Store(iter_end_ptr));

                let start_label = self.reserve_label("for start");
                self.set_label(start_label.clone());
                self.translate_commands(commands, variables, functions)?;
                self.load(iter_type);
                self.push(Instruction::Subtr(Pointer::Literal(1)));
                self.push(Instruction::Store(iter_ptr));
                self.push(Instruction::Subtr(iter_end_ptr));
                self.push(Instruction::Jneg(2));
                self.push(Instruction::Goto(start_label));

            }
            Command::FunctionCall(name, arguments) => {
                self.action_stack.push("FunctionCall".to_string());
                panic!("TODO");
            }
            Command::Read(id) => {
                self.action_stack.push("Read".to_string());
                self.read(variables.write_(Value::Identifier(id))?);
            }
            Command::Write(value) => {
                self.action_stack.push("Write".to_string());
                self.write(variables.read_(value)?);

            }
        }
        self.action_stack.pop();
        Ok(())
    }
}