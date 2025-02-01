mod structure;
mod variables;
mod procedures;
mod constants;
mod preprocessor;
mod intermediate;

use std::fs;
use std::str::FromStr;
use lalrpop_util::lalrpop_mod;
use intermediate::program_translator;
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
    // println!("{:?}", ret);
    let mut translator = program_translator::Translator::new();
    translator.translate(ret).unwrap();
    println!("Optimized Program:");
    translator.program.print();
    let code = translator.to_code();
    fs::write("output.mr", code).unwrap();
    // println!("{}", code);

}

#[test]
fn test(){
    let num = -15;
    let val = num >> 1;
    println!("{} ", val);
    let res = (val << 1)-num;
    println!("{}", res);
}
