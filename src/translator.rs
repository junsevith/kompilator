use crate::structure::Program;

struct Translator {

}

pub enum TranslationError {
    Unimplemented,
}

impl Translator {
    pub fn translate(program: Program) -> Result<String, TranslationError > {

        Err(TranslationError::Unimplemented)
    }
}