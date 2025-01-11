use std::fmt::{Debug, Formatter};
use std::ptr::write;

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

#[derive(Debug)]
pub enum ArgumentDecl {
    VariableArg(String),
    ArrayArg(String),
}

#[derive(Debug)]
pub enum Declaration {
    VariableDecl(String),
    ArrayDecl(String, i64, i64),
}

pub enum Command {
    Assign(Variable, Expression),
    If(Condition, Vec<Command>),
    IfElse(Condition, Vec<Command>, Vec<Command>),
    While(Condition, Vec<Command>),
    Repeat(Condition, Vec<Command>),
    For(String, Value, Value, Vec<Command>),
    ForDown(String, Value, Value, Vec<Command>),
    FunctionCall(String, Vec<String>),
    Read(Variable),
    Write(Value),
}



#[derive(Debug)]
pub enum Condition {
    Equal(Value, Value),
    NotEqual(Value, Value),
    Lesser(Value, Value),
    Greater(Value, Value),
    LesserEqual(Value, Value),
    GreaterEqual(Value, Value),
}

#[derive(Debug)]
pub enum Value {
    Literal(i64),
    Identifier(Variable),
}

#[derive(Debug)]
pub enum Variable {
    Variable(String),
    ArrayLit(String, i64),
    ArrayVar(String, String),
}

#[derive(Debug)]
pub enum Expression {
    Add(Value, Value),
    Subtract(Value, Value),
    Multiply(Value, Value),
    Divide(Value, Value),
    Modulo(Value, Value),
    Value(Value),
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
            Command::FunctionCall(name, args) => write!(f, "Call function {} with args {:?}", name, args),
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