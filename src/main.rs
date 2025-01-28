mod structure;
mod translator;
mod variables;
mod procedures;
mod constants;
mod preprocessor;
mod intermediate;

use std::str::FromStr;
use lalrpop_util::lalrpop_mod;
use crate::preprocessor::Preprocessor;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

// Spytać czy błędy muszą wyświetlać linijkę

fn main() {
    let parser = grammar::program_allParser::new();
    let file = std::fs::read_to_string("program2.imp").unwrap();
    let mut ret = parser.parse(&file).unwrap();
    let mut pre = Preprocessor::new();
    pre.process_program(&mut ret).unwrap();
    i64::from_str("123").unwrap();
    println!("{:?}", ret);
    println!("{:?}", pre);

}
