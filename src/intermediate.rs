use std::collections::HashMap;
use crate::structure::{Command, Identifier, Operation, Operator, Program, Value};
use crate::structure::Declaration::{ArrayDecl, VariableDecl};
use crate::variables::{Pointer, Type, VariableDictionary, VariableError};

#[derive(Debug)]
pub enum Instruction {
    Get(Pointer),
    Put(Pointer),
    Load(Pointer),
    Store(Pointer),
    Add(Pointer),
    Subtract(Pointer),
    // Set(i64),
    Half,
    Jump(Pointer),
    Jpos(i64),
    Jzero(i64),
    Jneg(i64),
    Goto(String),
    Label(String),
    Halt,
}
pub struct IntermediateProgram {
    literals: HashMap<i64, usize>,
    labels: Vec<String>,
    pub(crate) instructions: Vec<(Instruction, String)>,
    action_stack: Vec<String>,
}

#[derive(Debug)]
pub enum TranslationError {
    VariableError(VariableError),
    NegativeShift(String),
}

impl IntermediateProgram {
    pub fn new() -> Self {
        IntermediateProgram {
            literals: HashMap::new(),
            labels: Vec::new(),
            instructions: Vec::new(),
            action_stack: vec!["main".to_string()],
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push((instruction, self.action_stack.join(" ")));
    }

    pub(crate) fn translate_program(&mut self, commands: Program) -> Result<(), TranslationError> {
        let mut variables = VariableDictionary::new(10);
        for declaration in commands.declarations {
            variables.add(declaration).map_err(|error| { TranslationError::VariableError(error) })?;
        }
        for command in commands.commands {
            self.translate_command(command, &mut variables)?;
        }
        Ok(())
    }

    fn translate_command(&mut self, command: Command, variables: &mut VariableDictionary) -> Result<(), TranslationError> {
        match command {
            Command::Assign(variable, operation) => {
                self.translate_assign(variable, operation, variables)?;
            }
            Command::If(_, _) => {}
            Command::IfElse(_, _, _) => {}
            Command::While(_, _) => {}
            Command::Repeat(_, _) => {}
            Command::For(_, _, _, _) => {}
            Command::ForDown(_, _, _, _) => {}
            Command::FunctionCall(_, _) => {}
            Command::Read(id) => {
                self.action_stack.push("Read".to_string());
                self.read(variables.write_(Value::Identifier(id))?);
                self.action_stack.pop();
            }
            Command::Write(value) => {
                self.action_stack.push("Write".to_string());
                self.write(variables.read_(value)?);
                self.action_stack.pop();
            }
        }
        Ok(())
    }

    fn translate_assign(&mut self, variable: Identifier, operation: Operation, variables: &mut VariableDictionary) -> Result<(), TranslationError> {
        self.action_stack.push("Assignment".to_string());
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

                self.action_stack.pop();

            }
            Operator::Subtract => {
                self.action_stack.push("Subtraction".to_string());

                let first = variables.read_(operation.left)?;

                let second = variables.read_(operation.right)?;
                let second = self.prepare_pointer(second, 2);

                self.load(first);
                self.push(Instruction::Subtract(second));

                self.action_stack.pop();
            }
            Operator::Multiply => {


            }
            Operator::Divide => {
                self.action_stack.push("Division".to_string());



                self.action_stack.pop();
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
                        panic!("TODO")
                    }
                }

                self.action_stack.pop();
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
                    _ => panic!("Error in shift")
                };
                self.load(first);
                for _ in 0..second {
                    self.push(Instruction::Add(Pointer::Cell(0)));
                }
                self.action_stack.pop();
            }
            Operator::ShiftRight => {
                self.action_stack.push("Shift Right".to_string());

                let first = variables.read_(operation.left)?;
                let second = match variables.read_(operation.right)? {
                    Type::Variable(Pointer::Literal(x)) => x,
                    _ => panic!("Error in shift")
                };
                self.load(first);
                for _ in 0..second {
                    self.push(Instruction::Half);
                }
                self.action_stack.pop();
            }
        }

        self.push(Instruction::Store(destination));
        self.action_stack.pop();
        Ok(())
    }

    fn load(&mut self, variable: Type) {
        match variable {
            Type::Variable(pointer) => {
                self.push(Instruction::Load(pointer));
            }
            Type::Array(pointer1, pointer2) => {
                self.push(Instruction::Load(pointer1));
                self.push(Instruction::Add(pointer2));
                self.push(Instruction::Load(Pointer::IndirectCell(0)));
            }
        }
    }

    fn prepare_pointer(&mut self, variable: Type, registry: usize) -> Pointer {
        match variable {
            Type::Variable(pointer) => {
                pointer
            }
            Type::Array(pointer1, pointer2) => {
                self.push(Instruction::Load(pointer1));
                self.push(Instruction::Add(pointer2));
                self.push(Instruction::Store(Pointer::Cell(registry)));
                Pointer::IndirectCell(registry)

            }
        }
    }

    pub fn print(&self) {
        for (instruction, comment) in &self.instructions {
            println!("{:?} #{}", instruction, comment);
        }
    }

    fn neg_mod2(&mut self) {
        self.push(Instruction::Store(Pointer::Cell(2)));
        self.push(Instruction::Half);
        self.push(Instruction::Add(Pointer::Cell(0)));
        self.push(Instruction::Subtract(Pointer::Cell(2)));
    }

    fn neg(&mut self) {
        self.push(Instruction::Store(Pointer::Cell(2)));
        self.push(Instruction::Subtract(Pointer::Cell(2)));
        self.push(Instruction::Subtract(Pointer::Cell(2)));
    }

    fn write(&mut self, value: Type) {
        match value {
            Type::Variable(Pointer::Cell(_)) |
            Type::Variable(Pointer::Literal(_)) => {
                match value {
                    Type::Variable(pointer) => {
                        self.push(Instruction::Put(pointer));
                    }
                    _ => {
                        panic!("Error in write")
                    }
                }

            }
            _ => {
                self.load(value);
                self.push(Instruction::Put(Pointer::Cell(0)));
            }
        }
    }

    fn read(&mut self, value: Type) {
        match value {
            Type::Variable(Pointer::Cell(_)) |
            Type::Variable(Pointer::Literal(_)) => {
                match value {
                    Type::Variable(pointer) => {
                        self.push(Instruction::Get(pointer));
                    }
                    _ => {
                        panic!("Error in read")
                    }
                }

            }
            _ => {
                self.load(value);
                self.push(Instruction::Get(Pointer::Cell(0)));
            }
        }
    }
}

impl VariableDictionary {
    fn read_(&mut self, value: Value) -> Result<Type, TranslationError> {
        self.read(value).map_err(|error| { TranslationError::VariableError(error) })
    }

    fn write_(&mut self, value: Value) -> Result<Type, TranslationError> {
        self.write(value).map_err(|error| { TranslationError::VariableError(error) })
    }
}




#[test]
fn test() {
    let mut variables = VariableDictionary::new(4);
    variables.add(VariableDecl("a".to_string())).unwrap();
    let mut operation = Operation {
        operator: Operator::Add,
        left: Value::Identifier(Identifier::Variable("a".to_string())),
        right: Value::Literal(1),
    };
    let mut program = IntermediateProgram::new();
    program.translate_assign(Identifier::Variable("a".to_string()), operation, &mut variables).unwrap();

    println!("{:?}", program.instructions);
}

#[test]
fn test2() {
    let mut variables = VariableDictionary::new(4);
    variables.add(VariableDecl("a".to_string())).unwrap();
    variables.add(VariableDecl("b".to_string())).unwrap();
    variables.add(ArrayDecl("c".to_string(),1,5)).unwrap();
    let mut operation = Operation {
        operator: Operator::Add,
        left: Value::Literal(1),
        right: Value::Identifier(Identifier::ArrayVar("c".to_string(), "b".to_string())),
    };
    let mut program = IntermediateProgram::new();
    program.translate_assign(Identifier::Variable("a".to_string()), operation, &mut variables).unwrap();

    program.print();

}

#[test]
fn test3() {
    let mut variables = VariableDictionary::new(4);
    variables.add(VariableDecl("a".to_string())).unwrap();
    variables.add(VariableDecl("b".to_string())).unwrap();
    variables.write(Value::Identifier(Identifier::Variable("b".to_string()))).unwrap();
    let operation = Operation {
        operator: Operator::ShiftLeft,
        left: Value::Identifier(Identifier::Variable("b".to_string())),
        right: Value::Literal(2),
    };
    let mut program = IntermediateProgram::new();
    program.translate_assign(Identifier::Variable("a".to_string()), operation, &mut variables).unwrap();

    program.print();

}