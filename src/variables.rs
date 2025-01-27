use crate::constants::ConstantsHandler;
use crate::structure::{ArgumentDecl, Declaration, Identifier};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct VariableDictionary<'a> {
    variables: HashMap<String, Variable>,
    arrays: HashMap<String, Array>,
    literals: &'a ConstantsHandler,
    cell_counter: usize,
}
#[derive(Debug)]
struct Variable {
    cell: Pointer,
    init: bool,
}
#[derive(Debug)]
struct Array {
    offset: i64,
    offset_cell: Pointer,
    start: i64,
    length: usize,
}

#[derive(Debug, Copy, Clone)]
enum Type {
    Variable(Pointer),
    Array(Pointer, Pointer),
}

#[derive(Clone, Copy, Debug)]
enum Pointer {
    Cell(usize),
    IndirectCell(usize),
}

enum VariableError {
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
impl VariableDictionary<'_> {
    pub fn new(start: usize, literals: &ConstantsHandler) -> VariableDictionary {
        VariableDictionary {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            literals,
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
                self.arrays.insert(
                    name,
                    Array {
                        offset: self.cell_counter as i64 - from + 1,
                        offset_cell: Pointer::Cell(self.cell_counter),
                        start: from,
                        length: len,
                    },
                );
                self.cell_counter += len + 1;
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
                        offset: 0,
                        offset_cell: Pointer::IndirectCell(self.cell_counter),
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
                match array.offset_cell {
                    Pointer::Cell(_) => {
                        let translated_index = index - array.start;
                        if translated_index < 0 || translated_index >= array.length as i64 {
                            Err(VariableError::InvalidIndex(name, translated_index))
                        } else {
                            Ok(Type::Variable(Pointer::Cell(
                                (array.offset + index) as usize,
                            )))
                        }
                    }
                    Pointer::IndirectCell(_) => {
                        panic!("IndirectCell in ArrayLit");
                    }
                }
            }
            Identifier::ArrayVar(name, variable) => {
                let variable = self.get_variable(&variable)?;
                let array = self.get_array(&name)?;
                Ok(Type::Array(array.offset_cell, variable.cell))
            }
        }
    }

    pub fn read(&self, var: Identifier) -> Result<Type, VariableError> {
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

    pub fn write(&mut self, var: Identifier) -> Result<Type, VariableError> {
        match var {
            Identifier::Variable(name) => {
                self.get_variable(&name)?;
                self.variables.get_mut(&name).unwrap().init = true;
                self.pointer(Identifier::Variable(name))
            }
            something => self.pointer(something),
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
}
#[test]
pub fn test1() {
    let lit = ConstantsHandler::new(1);
    let mut dict = VariableDictionary::new(1, &lit);
    dict.add(Declaration::VariableDecl("a".to_string()))
        .unwrap();
    dict.add(Declaration::VariableDecl("b".to_string()))
        .unwrap();
    dict.add(Declaration::ArrayDecl("c".to_string(), 1, 10))
        .unwrap();
    dict.add(Declaration::ArrayDecl("d".to_string(), -10, 10))
        .unwrap();
    println!("{:?}", dict);
    let res = dict.pointer(Identifier::Variable("a".to_string())).unwrap();
    println!("{:?}", res);
    let res = dict
        .pointer(Identifier::ArrayLit("c".to_string(), 1))
        .unwrap();
    println!("{:?}", res);
    let res = dict
        .pointer(Identifier::ArrayLit("d".to_string(), -10))
        .unwrap();
    println!("{:?}", res);
    let res = dict
        .pointer(Identifier::ArrayVar("c".to_string(), "b".to_string()))
        .unwrap();
    println!("{:?}", res);
    let res = dict
        .pointer(Identifier::ArrayVar("d".to_string(), "a".to_string()))
        .unwrap();
    println!("{:?}", res);
}
