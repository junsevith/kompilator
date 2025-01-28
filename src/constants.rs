use std::collections::HashMap;

#[derive(Debug)]
pub struct ConstantsHandler {
    literals: HashMap<i64, usize>,
    stack: usize,
}

impl ConstantsHandler {
    pub fn new(stack: usize) -> Self {
        ConstantsHandler {
            literals: HashMap::new(),
            stack,
        }
    }

    pub fn add_literal(&mut self, literal: i64) {
        self.literals.insert(literal, self.stack);
        self.stack += 1;
    }

    pub fn get_literal(&self, literal: i64) -> usize {
        *self.literals.get(&literal).unwrap()
    }

    pub fn literal(&mut self, literal: i64) -> usize {
        *self.literals.entry(literal).or_insert({
            let end = self.stack;
            self.stack += 1;
            end
        })
    }
}
