use std::fmt::{Debug, Display, Formatter};

pub struct Program {
    pub procedures: Vec<Procedure>,
    pub declarations: Vec<Declaration>,
    pub commands: Vec<Command>,
}
pub struct Procedure {
    pub(crate) name: String,
    pub(crate) arguments: Vec<ArgumentDecl>,
    pub(crate) declarations: Vec<Declaration>,
    pub(crate) commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub enum ArgumentDecl {
    VariableArg(String),
    ArrayArg(String),
}

#[derive(Debug, Clone)]
pub enum Declaration {
    VariableDecl(String),
    ArrayDecl(String, i64, i64),
    ConstantDecl(String),
}

pub enum Command {
    Assign(Identifier, Operation),
    If(Condition, Vec<Command>),
    IfElse(Condition, Vec<Command>, Vec<Command>),
    While(Condition, Vec<Command>),
    Repeat(Condition, Vec<Command>),
    For(String, Value, Value, Vec<Command>),
    ForDown(String, Value, Value, Vec<Command>),
    FunctionCall(String, Vec<String>),
    Read(Identifier),
    Write(Value),
}

#[derive(Debug)]
pub struct Condition {
    pub(crate) operator: ConditionOperator,
    pub(crate) left: Value,
    pub(crate) right: Value,
}

#[derive(Debug)]
pub enum ConditionOperator {
    Equal,
    NotEqual,
    Lesser,
    Greater,
    LesserEqual,
    GreaterEqual,
}

#[derive(Debug)]
pub enum Value {
    Literal(i64),
    Identifier(Identifier),
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Variable(String),
    ArrayLit(String, i64),
    ArrayVar(String, String),
}

// #[derive(Debug)]
// pub enum Expression {
//     Add(Value, Value),
//     Subtract(Value, Value),
//     Multiply(Value, Value),
//     Divide(Value, Value),
//     Modulo(Value, Value),
//     Value(Value),
// }

#[derive(Debug)]
pub struct Operation {
    pub(crate) operator: Operator,
    pub(crate) left: Value,
    pub(crate) right: Value,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Value,
    ShiftLeft,
    ShiftRight,
}

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Procedures: \n")?;
        for procedure in &self.procedures {
            write!(f, "{:?}\n", procedure)?;
        }

        write!(f, "Declarations: \n")?;
        for declaration in &self.declarations {
            write!(f, "{:?}\n", declaration)?;
        }

        write!(f, "Commands: \n")?;
        for command in &self.commands {
            write!(f, "{:?}\n", command)?;
        }

        Ok(())
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Assign(var, expr) => write!(f, "Assign {:?} value {:?}", var, expr),
            Command::If(cond, commands) => {
                write!(f, "If condition {:?} then commands: \n", cond)?;
                for command in commands {
                    write!(f, "{:?}\n", command)?;
                }
                write!(f, "End if")?;
                Ok(())
            }
            Command::IfElse(cond, commands1, commands2) => {
                write!(f, "If condition {:?} then commands: \n", cond)?;
                for command in commands1 {
                    write!(f, "{:?}\n", command)?;
                }
                write!(f, "Else commands: \n")?;
                for command in commands2 {
                    write!(f, "{:?}\n", command)?;
                }
                write!(f, "End if")?;
                Ok(())
            }
            Command::While(cond, commands) => {
                write!(f, "While condition {:?} do commands: \n", cond)?;
                for command in commands {
                    write!(f, "{:?}\n", command)?;
                }
                write!(f, "End while")?;
                Ok(())
            }
            Command::Repeat(cond, commands) => {
                write!(f, "Repeat commands: \n")?;
                for command in commands {
                    write!(f, "{:?}\n", command)?;
                }
                write!(f, "Until condition {:?} \n", cond)
            }
            Command::For(var, from, to, commands) => {
                write!(f, "For {} from {:?} to {:?} do commands: \n", var, from, to)?;
                for command in commands {
                    write!(f, "{:?}\n", command)?;
                }
                write!(f, "End for")?;
                Ok(())
            }
            Command::ForDown(var, from, to, commands) => {
                write!(f, "For {} from {:?} downto {:?} do commands: \n", var, from, to)?;
                for command in commands {
                    write!(f, "{:?}\n", command)?;
                }
                write!(f, "End for")?;
                Ok(())
            }
            Command::FunctionCall(name, args) => write!(f, "Call function \"{}\" with args {:?}", name, args),
            Command::Read(var) => write!(f, "Read value to {:?}", var),
            Command::Write(val) => write!(f, "Write value {:?}", val),
        }
    }
}

impl Debug for Procedure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Procedure name: {} \n", self.name)?;
        write!(f, "Arguments: \n")?;
        for arg in &self.arguments {
            write!(f, "{:?}\n", arg)?;
        }

        write!(f, "Declarations: \n")?;
        for declaration in &self.declarations {
            write!(f, "{:?}\n", declaration)?;
        }

        write!(f, "Commands: \n")?;
        for command in &self.commands {
            write!(f, "{:?}\n", command)?;
        }

        write!(f, "End procedure\n")?;

        Ok(())
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.operator {
            Operator::Value => {
                write!(f, "{}", self.left)
            }
            _ => {
                write!(f, "{} {} {}", self.left, self.operator, self.right)
            }
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Modulo => write!(f, "%"),
            Operator::Value => write!(f, "Value"),
            Operator::ShiftLeft => write!(f, "<<"),
            Operator::ShiftRight => write!(f, ">>"),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl Display for ConditionOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionOperator::Equal => write!(f, "=="),
            ConditionOperator::NotEqual => write!(f, "!="),
            ConditionOperator::Lesser => write!(f, "<"),
            ConditionOperator::Greater => write!(f, ">"),
            ConditionOperator::LesserEqual => write!(f, "<="),
            ConditionOperator::GreaterEqual => write!(f, ">="),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(val) => {
                write!(f, "lit_{}", val)
            }
            Value::Identifier(Identifier::Variable(name)) => {
                write!(f, "var_{}", remove_program_things(name))
            }
            Value::Identifier(Identifier::ArrayLit(name, index)) => {
                write!(f, "arr_{}[lit {}]", remove_program_things(name), index)
            }
            Value::Identifier(Identifier::ArrayVar(name, var)) => {
                write!(f, "arr_{}[var {}]", remove_program_things(name), remove_program_things(var))
            }
        }
    }
}

fn remove_program_things(name: &str) -> String {
    let res = name.split("@").collect::<Vec<&str>>();
    res.last().unwrap().to_string()
}