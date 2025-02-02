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
        let destination = variables.write(Value::Identifier(variable))?;
        let destination = self.prepare_pointer(destination, 1);

        self.action_stack.push(format!("{}", operation));
        match &operation.operator {
            Operator::Add => {
                let first = variables.read(operation.left)?;

                let second = variables.read(operation.right)?;
                let second = self.prepare_pointer(second, 2);

                self.load(first);
                self.push(Instruction::Add(second));
            }
            Operator::Subtract => {
                let first = variables.read(operation.left)?;

                let second = variables.read(operation.right)?;
                let second = self.prepare_pointer(second, 2);

                self.load(first);
                self.push(Instruction::Subtr(second));
            }
            Operator::Multiply => {
                let first_type = variables.read(operation.left)?;
                let second_type = variables.read(operation.right)?;

                self.load(first_type);
                self.push(Instruction::Store(Pointer::Cell(6)));
                self.load(second_type);
                self.push(Instruction::Store(Pointer::Cell(7)));

                self.call_function(
                    "@multiply",
                    vec![],
                    variables,
                    functions
                )?;

            }
            Operator::Divide => {
                let first_type = variables.read(operation.left)?;
                let second_type = variables.read(operation.right)?;

                self.load(first_type);
                self.push(Instruction::Store(Pointer::Cell(6)));
                self.load(second_type);
                self.push(Instruction::Store(Pointer::Cell(7)));

                self.call_function(
                    "@divide",
                    vec![],
                    variables,
                    functions
                )?;

                self.load(Type::Variable(Pointer::Cell(4)));
            }
            Operator::Modulo => {
                match &operation.right {
                    Value::Literal(2) => {
                        let first = variables.read(operation.left)?;
                        self.load(first);
                        self.neg_mod2();
                        self.neg()
                    }
                    _ => {
                        let first_type = variables.read(operation.left)?;
                        let second_type = variables.read(operation.right)?;

                        self.load(first_type);
                        self.push(Instruction::Store(Pointer::Cell(6)));
                        self.load(second_type);
                        self.push(Instruction::Store(Pointer::Cell(7)));

                        self.call_function(
                            "@divide",
                            vec![],
                            variables,
                            functions
                        )?;

                        self.load(Type::Variable(Pointer::Cell(2)));
                    }
                }
            }
            Operator::Value => {
                let first = variables.read(operation.left)?;
                self.load(first);
            }
            Operator::ShiftLeft => {
                let first = variables.read(operation.left)?;
                let second = match variables.read(operation.right)? {
                    Type::Variable(Pointer::Literal(x)) => x,
                    _ => panic!("Error in shift"),
                };
                self.load(first);
                for _ in 0..second {
                    self.push(Instruction::Add(Pointer::Cell(0)));
                }
            }
            Operator::ShiftRight => {
                let first = variables.read(operation.left)?;
                let second = match variables.read(operation.right)? {
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
