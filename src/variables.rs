use crate::structure::{Declaration, Identifier};
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
    cell: usize,
    init: bool,
}
#[derive(Debug)]
struct Array {
    start: i64,
    length: usize,
    first_cell: usize,
}

#[derive(Debug)]
enum Pointer {
    Cell(usize),
    IndirectCell(i64, usize), // array pointer to which you should add index, Cell of variable that contains index,
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
impl VariableDictionary {
    pub fn new(start: usize) -> Self {
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
                        cell: self.cell_counter,
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
                        start: from,
                        length: len,
                        first_cell: self.cell_counter,
                    },
                );
                self.cell_counter += len;
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

    fn pointer(&self, var: Identifier) -> Result<Pointer, VariableError> {
        match var {
            Identifier::Variable(name) => {
                let variable = self.get_variable(&name)?;
                Ok(Pointer::Cell(variable.cell))
            }
            Identifier::ArrayLit(name, index) => {
                let array = self.get_array(&name)?;
                let index = index - array.start;
                if index < 0 || index >= array.length as i64 {
                    Err(VariableError::InvalidIndex(name, index))
                } else {
                    Ok(Pointer::Cell(array.first_cell + index as usize))
                }
            }
            Identifier::ArrayVar(name, variable) => {
                let variable = self.get_variable(&variable)?;
                let array = self.get_array(&name)?;
                let offset = array.first_cell as i64 - array.start;
                Ok(Pointer::IndirectCell(offset, variable.cell))
            }
        }
    }

    pub fn read(&self, var: Identifier) -> Result<Pointer, VariableError> {
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

    pub fn write(&mut self, var: Identifier) -> Result<Pointer, VariableError> {
        match var {
            Identifier::Variable(name) => {
                self.get_variable(&name)?;
                self.variables.get_mut(&name).unwrap().init = true;
                self.pointer(Identifier::Variable(name))
            }
            something => self.pointer(something),
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
    println!("{:?}", dict);
    let res = dict.pointer(Identifier::Variable("a".to_string())).unwrap();
    println!("{:?}", res);
    let res = dict
        .pointer(Identifier::ArrayLit("c".to_string(), 5))
        .unwrap();
    println!("{:?}", res);
    let res = dict
        .pointer(Identifier::ArrayLit("d".to_string(), -3))
        .unwrap();
    println!("{:?}", res);
    let res = dict
        .pointer(Identifier::ArrayVar("d".to_string(), "a".to_string()))
        .unwrap();
    println!("{:?}", res);
}
