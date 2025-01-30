mod structure;
mod translator;
mod variables;
mod procedures;
mod constants;
mod preprocessor;
mod intermediate;

use std::str::FromStr;
use lalrpop_util::lalrpop_mod;
use crate::intermediate::IntermediateProgram;
use crate::preprocessor::Preprocessor;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

// Spytać czy błędy muszą wyświetlać linijkę

fn main() {
    let parser = grammar::program_allParser::new();
    let file = std::fs::read_to_string("myprogram.imp").unwrap();
    let mut ret = parser.parse(&file).unwrap();
    println!("{:?}", ret);
    let mut pre = Preprocessor::new();
    pre.process_program(&mut ret).unwrap();
    // println!("{:?}", pre);
    // let mut intermediate = IntermediateProgram::new("main".to_string());
    // intermediate.translate_program(ret).expect("TODO: panic message");
    // intermediate.print();



}

#[test]
fn test(){
    let num = -15;
    let val = num >> 1;
    println!("{} ", val);
    let res = (val << 1)-num;
    println!("{}", res);
}
