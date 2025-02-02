use crate::intermediate::{CommandTranslator, Instruction, TranslationError};
use crate::structure::{Condition, ConditionOperator};
use crate::variables::{Pointer, VariableDictionary};

impl CommandTranslator {
    pub(crate) fn handle_condition(&mut self, condition: Condition, variables: &mut VariableDictionary, label: &String) -> Result<(), TranslationError> {
        self.action_stack.push(format!("Condition {}", condition));

        let Condition { left, right, operator } = condition;

        match operator {
            ConditionOperator::Equal => {
                let left = variables.read(left)?;
                let right = variables.read(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::Jzero(2));
                self.push(Instruction::Goto(label.clone()));
            }
            ConditionOperator::NotEqual => {
                let left = variables.read(left)?;
                let right = variables.read(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::GoZero(label.clone()));
            }
            ConditionOperator::Lesser => {
                let left = variables.read(left)?;
                let right = variables.read(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::Jneg(2));
                self.push(Instruction::Goto(label.clone()));
            }
            ConditionOperator::Greater => {
                let left = variables.read(left)?;
                let right = variables.read(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::Jpos(2));
                self.push(Instruction::Goto(label.clone()));
            }
            ConditionOperator::LesserEqual => {
                let left = variables.read(left)?;
                let right = variables.read(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::GoPos(label.clone()));
            }
            ConditionOperator::GreaterEqual => {
                let left = variables.read(left)?;
                let right = variables.read(right)?;
                let right = self.prepare_pointer(right, 2);

                self.load(left);
                if !matches!(right, Pointer::Literal(0)) {
                    self.push(Instruction::Subtr(right));
                }
                self.push(Instruction::GoNeg(label.clone()));
            }
        }

        self.action_stack.pop();
        Ok(())
    }
}