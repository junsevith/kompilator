use std::collections::HashMap;
use crate::intermediate::program_translator::Translator;
use crate::intermediate::{Instruction, InstructionLine, TranslationError};
use crate::structure::{Declaration, Identifier, Value};
use crate::variables::{Pointer, Type, VariableDictionary};

impl Translator {
    pub fn process_code(&mut self, variables: &mut VariableDictionary, main_label: String, literal_label: String) -> Result<(), TranslationError> {
        let mut literals_map = HashMap::new();
        let mut labels_map = HashMap::new();

        self.look_for_literals(&mut literals_map);

        let mut literals = vec![];
        for (literal, count) in literals_map.iter() {
            variables
                .add(Declaration::VariableDecl(format!("@lit{}", literal)))
                .unwrap();
            literals.push(*literal);
        }
        literals.sort();

        self.allocate_literals(literals, variables, main_label, literal_label)?;

        self.look_for_labels(&mut labels_map);

        self.swap_labels(variables, &mut labels_map)?;
        Ok(())
    }

    fn swap_labels(&mut self, variables: &mut VariableDictionary, labels_map: &mut HashMap<String, usize>) -> Result<(), TranslationError> {
        for (
            num,
            InstructionLine {
                instruction,
                comment,
                labels,
            },
        ) in self.program.instructions.iter_mut().enumerate()
        {
            match instruction {
                Instruction::Get(pointer)
                | Instruction::Put(pointer)
                | Instruction::Load(pointer)
                | Instruction::Store(pointer)
                | Instruction::Add(pointer)
                | Instruction::Subtr(pointer)
                | Instruction::Return(pointer) => match pointer {
                    Pointer::Cell(_) => {}
                    Pointer::IndirectCell(_) => {}
                    Pointer::Literal(lit) => {
                        let lit_type = variables.write_(Value::Identifier(
                            Identifier::Variable(format!("@lit{}", lit)),
                        ))?;
                        let lit_ptr = match lit_type {
                            Type::Variable(ptr) => ptr,
                            Type::Array(_, _) => {
                                panic!("Array in literals")
                            }
                        };
                        *pointer = lit_ptr;
                    }
                },
                Instruction::Half => {}
                Instruction::Jump(int)
                | Instruction::Jpos(int)
                | Instruction::Jzero(int)
                | Instruction::Jneg(int) => {}
                Instruction::Goto(label) => {
                    *comment = format!("{} Goto @[{}]", comment, label);
                    let label_num = labels_map.get(label).unwrap();
                    let count = *label_num as i64 - num as i64;
                    *instruction = Instruction::Jump(count);
                }
                Instruction::GoPos(label) => {
                    *comment = format!("{} GoPos @[{}]", comment, label);
                    let label_num = labels_map.get(label).unwrap();
                    let count = *label_num as i64 - num as i64;
                    *instruction = Instruction::Jpos(count);
                }
                Instruction::GoNeg(label) => {
                    *comment = format!("{} GoNeg @[{}]", comment, label);
                    let label_num = labels_map.get(label).unwrap();
                    let count = *label_num as i64 - num as i64;
                    *instruction = Instruction::Jneg(count);
                }
                Instruction::GoZero(label) => {
                    *comment = format!("{} GoZero @[{}]", comment, label);
                    let label_num = labels_map.get(label).unwrap();
                    let count = *label_num as i64 - num as i64;
                    *instruction = Instruction::Jzero(count);
                }

                Instruction::LoadCurrentLocation => {
                    panic!("LoadCurrentLocation should have been replaced by Load");
                }
                Instruction::Halt => {}
                Instruction::Set(_) => {}
            }
        }
        Ok(())
    }

    fn look_for_labels(&mut self, labels_map: &mut HashMap<String, usize>) {
        for (
            num,
            InstructionLine {
                instruction,
                comment,
                labels,
            },
        ) in self.program.instructions.iter_mut().enumerate()
        {
            for label in labels {
                labels_map
                    .insert(label.clone(), num)
                    .is_some()
                    .then(|| panic!("Label {} is defined multiple times", label));
            }
        }
    }

    fn look_for_literals(&mut self, literals_map: &mut HashMap<i64, usize>) {
        for (
            num,
            InstructionLine {
                instruction,
                comment,
                labels,
            },
        ) in self.program.instructions.iter_mut().enumerate()
        {
            match instruction {
                Instruction::Get(pointer)
                | Instruction::Put(pointer)
                | Instruction::Load(pointer)
                | Instruction::Store(pointer)
                | Instruction::Add(pointer)
                | Instruction::Subtr(pointer)
                | Instruction::Return(pointer) => match pointer {
                    Pointer::Cell(_) => {}
                    Pointer::IndirectCell(_) => {}
                    Pointer::Literal(lit) => {
                        let entry = literals_map.entry(*lit).or_insert(0usize);
                        *entry += 1;
                    }
                },
                Instruction::Half => {}
                Instruction::Jump(int)
                | Instruction::Jpos(int)
                | Instruction::Jzero(int)
                | Instruction::Jneg(int) => {}
                Instruction::Goto(label)
                | Instruction::GoPos(label)
                | Instruction::GoNeg(label)
                | Instruction::GoZero(label) => {}

                Instruction::LoadCurrentLocation => {
                    let val = num as i64 + 3;
                    *comment += " LoadCurrentLocation";
                    *instruction = Instruction::Load(Pointer::Literal(val));
                    let entry = literals_map.entry(val).or_insert(0usize);
                    *entry += 1;
                }
                Instruction::Halt => {}
                Instruction::Set(_) => {}
            }
        }
    }

    fn allocate_literals(&mut self, literals: Vec<i64>, variables: &mut VariableDictionary, main_label: String, literal_label: String) -> Result<(), TranslationError> {
        self.program.set_label(literal_label);
        for literal in literals {
            let typ = variables.write_(Value::Identifier(Identifier::Variable(format!("@lit{}", literal))))?;
            let ptr = match typ {
                Type::Variable(ptr) => ptr,
                Type::Array(_, _) => {
                    panic!("Array in literals")
                }
            };
            self.program.action_stack.push(format!("literal {}", literal));
            self.program.push(Instruction::Set(literal));
            self.program.push(Instruction::Store(ptr));
            self.program.action_stack.pop();
        }
        self.program.push(Instruction::Goto(main_label));
        Ok(())
    }
}