use std::collections::HashMap;
use std::mem;
use crate::structure::{Command, Condition, Declaration, Identifier, Operation, Operator, Program, Value};

#[derive(Debug)]
pub struct Preprocessor {
    literal_counter: HashMap<i64,usize>,
    function_counter: HashMap<String,usize>,
    found_iterators: Vec<String>,
}

impl Preprocessor {
    pub(crate) fn new() -> Self {
        Preprocessor {
            literal_counter: HashMap::new(),
            function_counter: HashMap::new(),
            found_iterators: Vec::new(),
        }
    }

    pub(crate) fn process_program(&mut self, program: &mut Program) {
        for procedure in program.procedures.iter_mut() {
            self.process_commands(&mut procedure.commands, false);
            let iters = mem::take(&mut self.found_iterators);
            for iter in iters {
                procedure.declarations.push(Declaration::VariableDecl(iter));
            }
        }
        self.process_commands(&mut program.commands, false);
    }

    fn process_commands(&mut self, commands: &mut Vec<Command>, in_loop: bool) {
        for command in commands {
            match command {
                Command::Assign(_, operation) => {
                    self.process_operation(operation);
                }
                Command::If(cond, commands)  => {
                    self.process_commands(commands, in_loop);
                    self.process_condition(cond);
                }
                Command::While(cond, commands) | Command::Repeat(cond, commands) => {
                    self.process_commands(commands, true);
                    self.process_condition(cond);
                }
                Command::IfElse(cond, commands, commands2) => {
                    self.process_commands(commands, in_loop);
                    self.process_commands(commands2, in_loop);
                    self.process_condition(cond);
                }
                Command::For(iterator, start, end, commands) | Command::ForDown(iterator, start, end, commands) => {
                    self.found_iterators.push(iterator.clone());
                    self.process_commands(commands, true);
                    self.process_value(start);
                    self.process_value(end);
                }
                Command::FunctionCall(name, _) => {
                    self.add_function_use(name);
                }
                Command::Read(_) => {}
                Command::Write(value) => {
                    self.process_value(value);
                }
            }
        }
    }

    fn process_value(&mut self, value: &Value) {
        match value {
            Value::Literal(val) => {
                let counter = self.literal_counter.entry(*val).or_insert(0);
                *counter += 1;
            }
            Value::Identifier(_) => {}
        }
    }

    fn add_function_use(&mut self, name: &str) {
        let counter = self.function_counter.entry(name.to_string()).or_insert(0);
        *counter += 1;
    }

    fn add_literal_use(&mut self, value: i64) {
        let counter = self.literal_counter.entry(value).or_insert(0);
        *counter += 1;
    }

    fn process_operation(&mut self, operation: &mut Operation) {
        match (&operation.left, &operation.right, &operation.operator) {
            (Value::Literal(lit), Value::Identifier(var), Operator::Multiply) |
            (Value::Identifier(var), Value::Literal(lit), Operator::Multiply) => {
                if lit.count_ones() == 1 {
                    let log = lit.trailing_zeros();
                    let mut new = Operation {
                        left: Value::Identifier(var.clone()),
                        right: Value::Literal(log as i64),
                        operator: Operator::ShiftLeft,
                    };
                    mem::swap(operation, &mut new);
                    let counter = self.function_counter.entry("@shift_left".to_string()).or_insert(0);
                    *counter += log as usize;
                } else {
                    self.add_function_use("@multiply");
                }
            }
            (Value::Identifier(var), Value::Literal(lit), Operator::Divide) => {
                if lit.count_ones() == 1 {
                    let log = lit.trailing_zeros();
                    let mut new = Operation {
                        left: Value::Identifier(var.clone()),
                        right: Value::Literal(log as i64),
                        operator: Operator::ShiftRight,
                    };
                    mem::swap(operation, &mut new);
                    let counter = self.function_counter.entry("@shift_right".to_string()).or_insert(0);
                    *counter += log as usize;
                } else {
                    self.add_function_use("@divide");
                }
            },
            (_,_,Operator::Multiply) => {
                self.add_function_use("@multiply");
            }
            (_,_,Operator::Divide) => {
                self.add_function_use("@divide");
            }
            _ => {}
        }
        self.process_value(&operation.left);
        self.process_value(&operation.right);

    }

    fn process_condition(&mut self, condition: &mut Condition) {
        self.process_value(&condition.left);
        self.process_value(&condition.right);
    }
}

#[test]
fn teest(){
    let mut op = Operation {
        left: Value::Identifier(Identifier::Variable("a".to_string())),
        right: Value::Literal(128),
        operator: Operator::Divide,
    };
    let mut preprocessor = Preprocessor::new();
    let _ = preprocessor.process_operation(&mut op);
    println!("{:?}", op);
}