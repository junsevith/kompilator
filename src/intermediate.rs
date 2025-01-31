use crate::procedures::{FunctionRepository, ProcedureHandler};
use crate::structure::Declaration::{ArrayDecl, VariableDecl};
use crate::structure::{Command, Condition, Identifier, Operation, Operator, Value};
use crate::variables::{Pointer, Type, VariableDictionary, VariableError};
use std::collections::HashMap;

type IntermediateProgram = Vec<(Instruction, String)>;
type LabelMap = HashMap<String, usize>;

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
    GoPos(String),
    GoNeg(String),
    GoZero(String),
    LoadCurrentLocation,
    Halt,
}
pub struct CommandTranslator {
    pub literal_counter: HashMap<i64, usize>,
    pub instructions: IntermediateProgram,
    pub labels: LabelMap,
    label_counter: usize,
    new_label: Option<String>,
    action_stack: Vec<String>,
    instruction_start: usize,
}

#[derive(Debug)]
pub enum TranslationError {
    VariableError(VariableError),
    NegativeShift(String),
}

impl CommandTranslator {
    pub fn new(name: String, instruction_start: usize) -> Self {
        CommandTranslator {
            literal_counter: HashMap::new(),
            labels: HashMap::new(),
            label_counter: 0,
            instructions: Vec::new(),
            action_stack: vec![name],
            new_label: None,
            instruction_start,
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        let mut comment = self.action_stack.join(" ");
        if let Some(label) = self.new_label.take() {
            comment = format!("{} @[ {} ]", comment, label);
        }
        self.instructions.push((instruction, comment));
    }

    pub fn translate_commands(
        &mut self,
        commands: Vec<Command>,
        variables: &mut VariableDictionary,
        functions: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        for command in commands {
            self.translate_command(command, variables, functions)?;
        }
        Ok(())
    }

    pub(crate) fn translate_command(
        &mut self,
        command: Command,
        variables: &mut VariableDictionary,
        functions: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
        match command {
            Command::Assign(variable, operation) => {
                self.translate_assign(variable, operation, variables, functions)?;
            }
            Command::If(condition, commands) => {
                self.action_stack.push("If".to_string());

                let label = self.reserve_label("if on false");
                self.handle_condition(condition, variables, &label, false);
                self.translate_commands(commands, variables, functions)?;
                self.set_label(label);

                self.action_stack.pop();
            }
            Command::IfElse(condition, yes, no) => {
                self.action_stack.push("IfElse".to_string());

                let end_label = self.reserve_label("ifelse end");
                let else_label = self.reserve_label("ifelse else");
                self.handle_condition(condition, variables, &else_label, false);
                self.translate_commands(yes, variables, functions)?;
                self.push(Instruction::Goto(end_label.clone()));
                self.set_label(else_label);
                self.translate_commands(no, variables, functions)?;
                self.set_label(end_label);

                self.action_stack.pop();
            }
            Command::While(condition, commands) => {
                self.action_stack.push("While".to_string());

                let start_label = self.reserve_label("while start");
                let end_label = self.reserve_label("while end");
                self.set_label(start_label.clone());
                self.handle_condition(condition, variables, &end_label, false);
                self.translate_commands(commands, variables, functions)?;
                self.push(Instruction::Goto(start_label));
                self.set_label(end_label);

                self.action_stack.pop();
            }
            Command::Repeat(condition, commands) => {
                self.action_stack.push("Repeat".to_string());

                let start_label = self.reserve_label("repeat start");
                self.set_label(start_label.clone());
                self.translate_commands(commands, variables, functions)?;
                self.handle_condition(condition, variables, &start_label, false);

                self.action_stack.pop();
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
                self.push(Instruction::Subtract(iter_end_ptr));
                self.push(Instruction::Jpos(2));
                self.push(Instruction::Goto(start_label));

                self.action_stack.pop();
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
                self.push(Instruction::Subtract(Pointer::Literal(1)));
                self.push(Instruction::Store(iter_ptr));
                self.push(Instruction::Subtract(iter_end_ptr));
                self.push(Instruction::Jneg(2));
                self.push(Instruction::Goto(start_label));

                self.action_stack.pop();
            }
            Command::FunctionCall(name, arguments) => {
                panic!("TODO");
            }
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

    fn translate_assign(
        &mut self,
        variable: Identifier,
        operation: Operation,
        variables: &mut VariableDictionary,
        functions: &mut FunctionRepository,
    ) -> Result<(), TranslationError> {
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
                self.action_stack.push("Multiplication".to_string());

                let first_type = variables.read_(operation.left)?;
                let second_type = variables.read_(operation.right)?;

                self.load(first_type);
                self.push(Instruction::Store(Pointer::Cell(2)));
                self.load(second_type);
                self.push(Instruction::Store(Pointer::Cell(3)));

                let res = functions
                    .get_mut("@multiply")
                    .unwrap()
                    .call(vec![], variables);

                self.action_stack.pop();
            }
            Operator::Divide => {
                self.action_stack.push("Division".to_string());

                let first_type = variables.read_(operation.left)?;
                let second_type = variables.read_(operation.right)?;

                self.load(first_type);
                self.push(Instruction::Store(Pointer::Cell(2)));
                self.load(second_type);
                self.push(Instruction::Store(Pointer::Cell(3)));

                functions
                    .get_mut("@divide")
                    .unwrap()
                    .call(vec![], variables);

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
                        let first_type = variables.read_(operation.left)?;
                        let second_type = variables.read_(operation.right)?;

                        self.load(first_type);
                        self.push(Instruction::Store(Pointer::Cell(2)));
                        self.load(second_type);
                        self.push(Instruction::Store(Pointer::Cell(3)));

                        functions
                            .get_mut("@modulo")
                            .unwrap()
                            .call(vec![], variables);
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
                    _ => panic!("Error in shift"),
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
                    _ => panic!("Error in shift"),
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
            Type::Variable(pointer) => pointer,
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
            Type::Variable(Pointer::Cell(_)) | Type::Variable(Pointer::Literal(_)) => match value {
                Type::Variable(pointer) => {
                    self.push(Instruction::Put(pointer));
                }
                _ => {
                    panic!("Error in write")
                }
            },
            _ => {
                self.load(value);
                self.push(Instruction::Put(Pointer::Cell(0)));
            }
        }
    }

    fn read(&mut self, value: Type) {
        match value {
            Type::Variable(Pointer::Cell(_)) | Type::Variable(Pointer::Literal(_)) => match value {
                Type::Variable(pointer) => {
                    self.push(Instruction::Get(pointer));
                }
                _ => {
                    panic!("Error in read")
                }
            },
            _ => {
                self.load(value);
                self.push(Instruction::Get(Pointer::Cell(0)));
            }
        }
    }

    pub fn reserve_label(&mut self, name: &str) -> String {
        let label = format!(
            "{} {} {}",
            self.action_stack.first().unwrap(),
            name,
            self.label_counter
        );
        self.label_counter += 1;
        label
    }

    pub fn set_label(&mut self, label: String) -> String {
        self.labels.insert(
            label.clone(),
            self.instructions.len() + self.instruction_start,
        );
        match &mut self.new_label {
            None => {
                self.new_label = Some(label.clone());
            }
            Some(old) => {
                *old = old.clone() + " | " + &label;
            }
        }
        label
    }

    fn handle_condition(&mut self, condition: Condition, variables: &mut VariableDictionary, label: &String, when_jump: bool) {
        //We jump when false
        // panic!("TODO");
        self.push(Instruction::Goto("Not implemented".to_string()));
    }

    pub fn where_we_finished(&self) -> usize {
        self.instructions.len() + self.instruction_start
    }

    pub fn merge(&mut self, mut other: CommandTranslator) {
        if self.instructions.len() != other.instruction_start {
            panic!("Error in merging");
        }
        self.instructions.append(&mut other.instructions);
        self.labels.extend(other.labels);
        other.literal_counter.into_iter().for_each(|(key, value)| {
            let entry = self.literal_counter.entry(key).or_insert(0);
            *entry += value;
        });

    }
}

impl VariableDictionary {
    fn read_(&mut self, value: Value) -> Result<Type, TranslationError> {
        self.read(value)
            .map_err(|error| TranslationError::VariableError(error))
    }

    fn write_(&mut self, value: Value) -> Result<Type, TranslationError> {
        self.write(value)
            .map_err(|error| TranslationError::VariableError(error))
    }
}

#[test]
fn test() {
    let mut variables = VariableDictionary::new(4);
    variables.add(VariableDecl("a".to_string())).unwrap();
    let mut functions = FunctionRepository::new();
    let operation = Operation {
        operator: Operator::Add,
        left: Value::Identifier(Identifier::Variable("a".to_string())),
        right: Value::Literal(1),
    };
    let mut program = CommandTranslator::new("Test1".to_string(), 0);
    program
        .translate_assign(
            Identifier::Variable("a".to_string()),
            operation,
            &mut variables,
            &mut functions,
        )
        .unwrap();

    println!("{:?}", program.instructions);
}

#[test]
fn test2() {
    let mut functions = FunctionRepository::new();
    let mut variables = VariableDictionary::new(10);
    variables.add(VariableDecl("a".to_string())).unwrap();
    variables.add(VariableDecl("b".to_string())).unwrap();
    variables.add(ArrayDecl("c".to_string(), 1, 5)).unwrap();
    let operation = Operation {
        operator: Operator::Add,
        left: Value::Literal(1),
        right: Value::Identifier(Identifier::ArrayVar("c".to_string(), "b".to_string())),
    };
    let mut program = CommandTranslator::new("Test2".to_string(), 0);
    program
        .translate_assign(
            Identifier::Variable("a".to_string()),
            operation,
            &mut variables,
            &mut functions,
        )
        .unwrap();

    program.print();
}

#[test]
fn test3() {
    let mut functions = FunctionRepository::new();
    let mut variables = VariableDictionary::new(4);
    variables.add(VariableDecl("a".to_string())).unwrap();
    variables.add(VariableDecl("b".to_string())).unwrap();
    variables
        .write(Value::Identifier(Identifier::Variable("b".to_string())))
        .unwrap();
    let operation = Operation {
        operator: Operator::ShiftLeft,
        left: Value::Identifier(Identifier::Variable("b".to_string())),
        right: Value::Literal(2),
    };
    let mut program = CommandTranslator::new("Test3".to_string(), 0);
    program
        .translate_assign(
            Identifier::Variable("a".to_string()),
            operation,
            &mut variables,
            &mut functions,
        )
        .unwrap();
    program.set_label("end".to_string());
    program.set_label("dupa".to_string());
    program.push(Instruction::Halt);

    program.print();

    println!("{:?}", program.labels);
}
