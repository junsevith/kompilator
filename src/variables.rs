use crate::structure::{ArgumentDecl, Declaration, Identifier, Value};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct VariableDictionary {
    variables: HashMap<String, Variable>,
    arrays: HashMap<String, Array>,
    cell_counter: usize,
}
#[derive(Debug)]
struct Variable {
    cell: Pointer,
    init: bool,
}
#[derive(Debug)]
struct Array {
    offset: Pointer,
    start: i64,
    length: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Variable(Pointer),
    Array(Pointer, Pointer),
}

#[derive(Clone, Copy, Debug)]
pub enum Pointer {
    Cell(usize),
    IndirectCell(usize),
    Literal(i64)
}

pub enum VariableError {
    ArrayCollision(String),
    VariableCollision(String),
    ArrayMixup(String),
    VariableMixup(String),
    NoArray(String),
    NoVariable(String),
    InvalidIndex(String, i64),
    NotInitialized(String),
}
impl Debug for VariableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableError::ArrayCollision(name) => {
                write!(f, "There already exists declared array with name {}", name)
            }
            VariableError::VariableCollision(name) => write!(
                f,
                "There already exists declared variable with name {}",
                name
            ),
            VariableError::ArrayMixup(name) => {
                write!(f, "Tried to use {} as variable but it is an array", name)
            }
            VariableError::VariableMixup(name) => {
                write!(f, "Tried to use {} as array but it is a variable", name)
            }
            VariableError::NoArray(name) => {
                write!(f, "No array with name {} declared", name)
            }
            VariableError::NoVariable(name) => {
                write!(f, "No variable with name {} declared", name)
            }
            VariableError::InvalidIndex(name, index) => {
                write!(f, "Invalid index {} for array {}", index, name)
            }
            VariableError::NotInitialized(name) => {
                write!(f, "Variable {} was not initialized", name)
            }
        }
    }
}
impl VariableDictionary {
    pub fn new(start: usize) -> VariableDictionary {
        VariableDictionary {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            cell_counter: start,
        }
    }

    fn check_name(&self, name: &str) -> Result<(), VariableError> {
        if self.variables.contains_key(name) {
            Err(VariableError::VariableCollision(name.to_string()))
        } else if self.arrays.contains_key(name) {
            Err(VariableError::ArrayCollision(name.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn add(&mut self, var: Declaration) -> Result<(), VariableError> {
        match var {
            Declaration::VariableDecl(name) => {
                self.check_name(&name)?;
                self.variables.insert(
                    name,
                    Variable {
                        cell: Pointer::Cell(self.cell_counter),
                        init: false,
                    },
                );
                self.cell_counter += 1;
            }
            Declaration::ArrayDecl(name, from, to) => {
                self.check_name(&name)?;
                let len = (to - from + 1) as usize;
                let offset = self.cell_counter as i64 - from;
                self.arrays.insert(
                    name,
                    Array {
                        offset: Pointer::Literal(offset),
                        start: from,
                        length: len,
                    },
                );
                self.cell_counter += len;
            }
        }
        Ok(())
    }

    pub fn add_argument(&mut self, var: ArgumentDecl) -> Result<(), VariableError> {
        match var {
            ArgumentDecl::VariableArg(name) => {
                self.check_name(&name)?;
                self.variables.insert(
                    name,
                    Variable {
                        cell: Pointer::IndirectCell(self.cell_counter),
                        init: true,
                    },
                );
                self.cell_counter += 1;
            }
            ArgumentDecl::ArrayArg(name) => {
                self.check_name(&name)?;
                self.arrays.insert(
                    name,
                    Array {
                        offset: Pointer::IndirectCell(self.cell_counter),
                        start: 0,
                        length: 0,
                    },
                );
                self.cell_counter += 1;
            }
        }
        Ok(())
    }

    fn get_variable(&self, name: &str) -> Result<&Variable, VariableError> {
        match self.variables.get(name) {
            None => {
                if self.arrays.contains_key(name) {
                    Err(VariableError::VariableMixup(name.to_string()))
                } else {
                    Err(VariableError::NoVariable(name.to_string()))
                }
            }
            Some(variable) => Ok(variable),
        }
    }

    fn get_array(&self, name: &str) -> Result<&Array, VariableError> {
        match self.arrays.get(name) {
            None => {
                if self.variables.contains_key(name) {
                    Err(VariableError::ArrayMixup(name.to_string()))
                } else {
                    Err(VariableError::NoArray(name.to_string()))
                }
            }
            Some(array) => Ok(array),
        }
    }

    fn pointer(&self, var: Identifier) -> Result<Type, VariableError> {
        match var {
            Identifier::Variable(name) => {
                let variable = self.get_variable(&name)?;
                Ok(Type::Variable(variable.cell))
            }
            Identifier::ArrayLit(name, index) => {
                let array = self.get_array(&name)?;
                match array.offset {
                    Pointer::Literal(lit) => {
                        let translated_index = index - array.start;
                        if translated_index < 0 || translated_index >= array.length as i64 {
                            Err(VariableError::InvalidIndex(name, translated_index))
                        } else {
                            Ok(Type::Variable(Pointer::Cell(
                                (lit + index) as usize,
                            )))
                        }
                    }
                    Pointer::IndirectCell(_) => {
                        Ok(Type::Array(array.offset, Pointer::Literal(index)))
                    }
                    _ => {
                        panic!("Cell in ArrayLit");
                    }
                }
            }
            Identifier::ArrayVar(name, variable) => {
                let variable = self.get_variable(&variable)?;
                let array = self.get_array(&name)?;
                Ok(Type::Array(array.offset, variable.cell))
            }
        }
    }

    pub fn read_identifier(&self, var: Identifier) -> Result<Type, VariableError> {
        match var {
            Identifier::Variable(name) => {
                let variable = self.get_variable(&name)?;
                if variable.init {
                    self.pointer(Identifier::Variable(name))
                } else {
                    Err(VariableError::NotInitialized(name))
                }
            }
            something => self.pointer(something),
        }
    }

    pub fn write_identifier(&mut self, var: Identifier) -> Result<Type, VariableError> {
        match var {
            Identifier::Variable(name) => {
                self.get_variable(&name)?;
                self.variables.get_mut(&name).unwrap().init = true;
                self.pointer(Identifier::Variable(name))
            }
            something => self.pointer(something),
        }
    }

    pub fn read(&self, var: Value) -> Result<Type, VariableError> {
        match var {
            Value::Literal(lit) => {
                Ok(Type::Variable(Pointer::Literal(lit)))
            }
            Value::Identifier(identifier) => {
                self.read_identifier(identifier)
            }
        }
    }

    pub fn write(&mut self, var: Value) -> Result<Type, VariableError> {
        match var {
            Value::Literal(lit) => {
                Ok(Type::Variable(Pointer::Literal(lit)))
            }
            Value::Identifier(identifier) => {
                self.write_identifier(identifier)
            }
        }
    }

    pub fn init_variables(&mut self, vars: Vec<Declaration>) {
        for var in vars {
            self.add(var).unwrap();
        }
    }

    pub fn init_arguments(&mut self, args: Vec<ArgumentDecl>) {
        for arg in args {
            self.add_argument(arg).unwrap();
        }
    }

    pub fn show_allocation(&self) {
        let mut memory = vec!["".to_string(); self.cell_counter];
        for (name, var) in &self.variables {
            match var.cell {
                Pointer::Cell(cell) => {
                    memory[cell] = format!("var {}", name);
                }
                Pointer::IndirectCell(cell) => {
                    memory[cell] = format!("var arg {}", name);
                }
                Pointer::Literal(_) => {
                    panic!("Literal in variable");
                }
            }
        }
        for (name, arr) in &self.arrays {
            match arr.offset {
                Pointer::Cell(_) => {
                    panic!("Cell in array");
                }
                Pointer::IndirectCell(cell) => {
                    memory[cell] = format!("arr arg {}", name);
                }
                Pointer::Literal(offset) => {
                    let start = arr.start;
                    for i in start..(start+arr.length as i64) {
                        memory[(offset + i) as usize] = format!("arr {}", name);
                    }
                }
            }
        }
        for m in memory {
            println!("{}", m);
        }
    }
}
#[test]
pub fn test1() {
    let mut dict = VariableDictionary::new(1);
    dict.add(Declaration::VariableDecl("a".to_string()))
        .unwrap();
    dict.add(Declaration::VariableDecl("b".to_string()))
        .unwrap();
    dict.add(Declaration::ArrayDecl("c".to_string(), 1, 10))
        .unwrap();
    dict.add(Declaration::ArrayDecl("d".to_string(), -10, 10))
        .unwrap();
    dict.add_argument(ArgumentDecl::VariableArg("f".to_string())).unwrap();
    dict.add_argument(ArgumentDecl::ArrayArg("g".to_string())).unwrap();
    println!("{:?}", dict);

    // let res = dict.pointer(Identifier::Variable("a".to_string())).unwrap();
    // println!("{:?}", res);
    // let res = dict
    //     .pointer(Identifier::ArrayLit("c".to_string(), 1))
    //     .unwrap();
    // println!("{:?}", res);
    // let res = dict
    //     .pointer(Identifier::ArrayLit("d".to_string(), -10))
    //     .unwrap();
    // println!("{:?}", res);
    // let res = dict
    //     .pointer(Identifier::ArrayVar("c".to_string(), "b".to_string()))
    //     .unwrap();
    // println!("{:?}", res);
    // let res = dict
    //     .pointer(Identifier::ArrayVar("d".to_string(), "a".to_string()))
    //     .unwrap();
    // println!("{:?}", res);

    dict.show_allocation()
}
