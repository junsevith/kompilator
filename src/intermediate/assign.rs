use crate::intermediate::{CommandTranslator, Instruction, TranslationError};
use crate::procedures::procedures::FunctionRepository;
use crate::structure::{Identifier, Operation, Operator, Value};
use crate::variables::{Pointer, Type, VariableDictionary};

impl CommandTranslator {
    pub(crate) fn translate_assign(
        &mut self,
        variable: Identifier,
        operation: Operation,
        variables: &mut VariableDictionary,
        functions: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        let destination = variables.write_(Value::Identifier(variable))?;
        let destination = self.prepare_pointer(destination, 1);

        match &operation.operator {
            Operator::Add => {
                self.action_stack.push("Addition".to_string());

                let first = variables.read_(operation.left)?;

                let second = variables.read_(operation.right)?;
                let second = self.prepare_pointer(second, 2);

                self.load(first);
                self.push(Instruction::Add(second));
            }
            Operator::Subtract => {
                self.action_stack.push("Subtraction".to_string());

                let first = variables.read_(operation.left)?;

                let second = variables.read_(operation.right)?;
                let second = self.prepare_pointer(second, 2);

                self.load(first);
                self.push(Instruction::Subtr(second));
            }
            Operator::Multiply => {
                self.action_stack.push("Multiplication".to_string());

                let first_type = variables.read_(operation.left)?;
                let second_type = variables.read_(operation.right)?;

                self.load(first_type);
                self.push(Instruction::Store(Pointer::Cell(6)));
                self.load(second_type);
                self.push(Instruction::Store(Pointer::Cell(7)));

                let res = functions
                    .get_mut("@multiply")
                    .unwrap()
                    .call(vec![], variables);
            }
            Operator::Divide => {
                self.action_stack.push("Division".to_string());

                let first_type = variables.read_(operation.left)?;
                let second_type = variables.read_(operation.right)?;

                self.load(first_type);
                self.push(Instruction::Store(Pointer::Cell(6)));
                self.load(second_type);
                self.push(Instruction::Store(Pointer::Cell(7)));

                functions
                    .get_mut("@divide")
                    .unwrap()
                    .call(vec![], variables);

                self.load(Type::Variable(Pointer::Cell(4)));
            }
            Operator::Modulo => {
                self.action_stack.push("Modulo".to_string());

                match &operation.right {
                    Value::Literal(2) => {
                        let first = variables.read_(operation.left)?;
                        self.load(first);
                        self.neg_mod2();
                        self.neg()
                    }
                    _ => {
                        let first_type = variables.read_(operation.left)?;
                        let second_type = variables.read_(operation.right)?;

                        self.load(first_type);
                        self.push(Instruction::Store(Pointer::Cell(6)));
                        self.load(second_type);
                        self.push(Instruction::Store(Pointer::Cell(7)));

                        functions
                            .get_mut("@divide")
                            .unwrap()
                            .call(vec![], variables);

                        self.load(Type::Variable(Pointer::Cell(2)));
                    }
                }
            }
            Operator::Value => {
                let first = variables.read_(operation.left)?;
                self.load(first);
            }
            Operator::ShiftLeft => {
                self.action_stack.push("Shift Left".to_string());

                let first = variables.read_(operation.left)?;
                let second = match variables.read_(operation.right)? {
                    Type::Variable(Pointer::Literal(x)) => x,
                    _ => panic!("Error in shift"),
                };
                self.load(first);
                for _ in 0..second {
                    self.push(Instruction::Add(Pointer::Cell(0)));
                }
            }
            Operator::ShiftRight => {
                self.action_stack.push("Shift Right".to_string());

                let first = variables.read_(operation.left)?;
                let second = match variables.read_(operation.right)? {
                    Type::Variable(Pointer::Literal(x)) => x,
                    _ => panic!("Error in shift"),
                };
                self.load(first);
                for _ in 0..second {
                    self.push(Instruction::Half);
                }

            }
        }
        self.action_stack.pop();

        self.push(Instruction::Store(destination));
        Ok(())
    }
}