use crate::intermediate::TranslationError;
use crate::structure::{Command, Condition, Identifier, Operation, Value};
use std::collections::HashMap;

pub fn swap_commands(commands: &mut Vec<Command>, variable_map: &HashMap<String, String>) -> Result<(), TranslationError> {
    for command in commands {
        match command {
            Command::Assign(identifier, Operation{ operator, left, right }) => {
                swap_identifier(identifier, variable_map)?;
                swap_values(left, variable_map)?;
                swap_values(right, variable_map)?;
            }
            Command::If(Condition{ operator, left, right }, commands) => {
                swap_values(left, variable_map)?;
                swap_values(right, variable_map)?;
                swap_commands(commands, variable_map)?;
            }
            Command::IfElse(Condition{ operator, left, right }, commands1, commands2) => {
                swap_values(left, variable_map)?;
                swap_values(right, variable_map)?;
                swap_commands(commands1, variable_map)?;
                swap_commands(commands2, variable_map)?;
            }
            Command::While(Condition{ operator, left, right }, commands) => {
                swap_values(left, variable_map)?;
                swap_values(right, variable_map)?;
                swap_commands(commands, variable_map)?;
            }
            Command::Repeat(Condition{ operator, left, right }, commands) => {
                swap_values(left, variable_map)?;
                swap_values(right, variable_map)?;
                swap_commands(commands, variable_map)?;
            }
            Command::For(iter, first, second, commands) => {
                *iter = variable_map.get(iter).unwrap().clone();
                swap_values(first, variable_map)?;
                swap_values(second, variable_map)?;
                swap_commands(commands, variable_map)?;
            }
            Command::ForDown(iter, first, second, commands) => {
                *iter = variable_map.get(iter).unwrap().clone();
                swap_values(first, variable_map)?;
                swap_values(second, variable_map)?;
                swap_commands(commands, variable_map)?;
            }
            Command::FunctionCall(name, arguments) => {
                for argument in arguments {
                    *argument = variable_map.get(argument).unwrap().clone();
                }
            }
            Command::Read(identifier) => {
                swap_identifier(identifier, variable_map)?;
            }
            Command::Write(value) => {
                swap_values(value, variable_map)?;
            }
        }
    }
    Ok(())
}

fn swap_values(value: &mut Value, variable_map: &HashMap<String, String>) -> Result<(), TranslationError> {
    match value {
        Value::Literal(_) => {}
        Value::Identifier(identifier) => {
            swap_identifier(identifier, variable_map)?;
        }
    }
    Ok(())
}

fn swap_identifier(identifier: &mut Identifier, variable_map: &HashMap<String, String>) -> Result<(), TranslationError> {
    match identifier {
        Identifier::Variable(name) => {
            let new_name = match variable_map.get(name) {
                None => format!("@unid@{}", name),
                Some(name) => name.clone()
            };
            *name = new_name;
        }
        Identifier::ArrayLit(name, _) => {
            let new_name = match variable_map.get(name) {
                None => format!("@unid@{}", name),
                Some(name) => name.clone()
            };
            *name = new_name;
        }
        Identifier::ArrayVar(name, variable) => {
            let new_name = match variable_map.get(name) {
                None => format!("@unid@{}", name),
                Some(name) => name.clone()
            };
            *name = new_name;
            let new_variable = match variable_map.get(variable) {
                None => format!("@unid@{}", variable),
                Some(name) => name.clone()
            };
            *variable = new_variable;
        }
    }
    Ok(())
}