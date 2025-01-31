mod structure;
mod translator;
mod variables;
mod procedures;
mod constants;
mod preprocessor;
mod intermediate;
mod program_translator;

use std::str::FromStr;
use lalrpop_util::lalrpop_mod;
use crate::intermediate::CommandTranslator;
use crate::preprocessor::Preprocessor;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

// Spytać czy błędy muszą wyświetlać linijkę

fn main() {
    let parser = grammar::program_allParser::new();
    let file = std::fs::read_to_string("program0.imp").unwrap();
    let mut ret = parser.parse(&file).unwrap();
    println!("{:?}", ret);
    let mut translator = program_translator::Translator::new();
    translator.translate(ret);
    translator.program.print()



}

#[test]
fn test(){
    let num = -15;
    let val = num >> 1;
    println!("{} ", val);
    let res = (val << 1)-num;
    println!("{}", res);
}
