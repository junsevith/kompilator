use crate::intermediate::{CommandTranslator, Instruction, TranslationError};
use crate::structure::{Condition, ConditionOperator};
use crate::variables::{Pointer, Type, VariableDictionary};

impl CommandTranslator {
    pub(crate) fn handle_condition(&mut self, condition: Condition, variables: &mut VariableDictionary, label: &String) -> Result<(), TranslationError> {
        let Condition { left, right, operator } = condition;
        self.action_stack.push("Condition".to_string());

        match operator {
            ConditionOperator::Equal => {
                self.action_stack.push("Equal".to_string());
                let left = variables.read_(left)?;
                let right = variables.read_(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::Jzero(2));
                self.push(Instruction::Goto(label.clone()));
            }
            ConditionOperator::NotEqual => {
                self.action_stack.push("NotEqual".to_string());
                let left = variables.read_(left)?;
                let right = variables.read_(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::GoZero(label.clone()));
            }
            ConditionOperator::Lesser => {
                self.action_stack.push("Lesser".to_string());
                let left = variables.read_(left)?;
                let right = variables.read_(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::Jneg(2));
                self.push(Instruction::Goto(label.clone()));
            }
            ConditionOperator::Greater => {
                self.action_stack.push("Greater".to_string());
                let left = variables.read_(left)?;
                let right = variables.read_(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::Jpos(2));
                self.push(Instruction::Goto(label.clone()));
            }
            ConditionOperator::LesserEqual => {
                self.action_stack.push("LeEq".to_string());
                let left = variables.read_(left)?;
                let right = variables.read_(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::GoPos(label.clone()));
            }
            ConditionOperator::GreaterEqual => {
                self.action_stack.push("GeEq".to_string());
                let left = variables.read_(left)?;
                let right = variables.read_(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::GoNeg(label.clone()));
            }
        }

        self.action_stack.pop();
        self.action_stack.pop();
        Ok(())
    }
}