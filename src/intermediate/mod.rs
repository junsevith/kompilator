mod assign;
mod command;
pub mod program_translator;
mod condition;
mod scanner;
mod to_code;

use crate::structure::Declaration::{ArrayDecl, VariableDecl};
use crate::structure::{Command, Condition, Identifier, Operation, Operator, Value};
use crate::variables::{Pointer, Type, VariableDictionary, VariableError};
use std::fmt::{Debug, Display};
use std::mem;
use crate::preprocessor::StaticAnalysisError;
use crate::procedures::procedures::FunctionRepository;

pub struct InstructionLine {
    pub instruction: Instruction,
    pub comment: String,
    pub labels: Vec<String>,
}

impl Debug for InstructionLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instruction = format!("{:?}", self.instruction);
        let labels = self.labels.join(" ");
        write!(f, "{:<30} @{:<20} #{} ", instruction, labels, self.comment)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Get(Pointer),
    Put(Pointer),
    Load(Pointer),
    Store(Pointer),
    Add(Pointer),
    Subtr(Pointer),
    Half,
    Set(i64),
    Jump(i64),
    Jpos(i64),
    Jzero(i64),
    Jneg(i64),
    Goto(String),
    GoPos(String),
    GoNeg(String),
    GoZero(String),
    Return(Pointer),
    LoadCurrentLocation,
    Halt,
}
pub struct CommandTranslator {
    pub instructions: Vec<InstructionLine>,
    label_counter: usize,
    next_labels: Vec<String>,
    pub(crate) action_stack: Vec<String>,
    instruction_start: usize,
}

#[derive(Debug)]
pub enum TranslationError {
    VariableError(VariableError),
    PreprocessorError(StaticAnalysisError),
    NegativeShift(String),
}

impl CommandTranslator {
    pub fn new(name: String, instruction_start: usize) -> Self {
        CommandTranslator {
            label_counter: 0,
            instructions: Vec::new(),
            action_stack: vec![name],
            next_labels: Vec::new(),
            instruction_start,
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        let instruction = InstructionLine {
            instruction,
            comment: self.action_stack.join(" "),
            labels: mem::take(&mut self.next_labels),
        };
        self.instructions.push(instruction);
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

    pub fn prepare_pointer(&mut self, variable: Type, registry: usize) -> Pointer {
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
        for line in &self.instructions {
            println!("{:?}", line);
        }
    }

    fn neg_mod2(&mut self) {
        self.push(Instruction::Store(Pointer::Cell(2)));
        self.push(Instruction::Half);
        self.push(Instruction::Add(Pointer::Cell(0)));
        self.push(Instruction::Subtr(Pointer::Cell(2)));
    }

    fn neg(&mut self) {
        self.push(Instruction::Store(Pointer::Cell(2)));
        self.push(Instruction::Subtr(Pointer::Cell(2)));
        self.push(Instruction::Subtr(Pointer::Cell(2)));
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

    pub fn set_label(&mut self, label: String) {
        self.next_labels.push(label);
    }

    pub fn where_we_finished(&self) -> usize {
        self.instructions.len() + self.instruction_start
    }

    pub fn merge(&mut self, mut other: CommandTranslator) {
        if self.where_we_finished() != other.instruction_start {
            panic!("Error in merging");
        }
        self.instructions.append(&mut other.instructions);
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

}
