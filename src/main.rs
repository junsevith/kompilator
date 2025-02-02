mod structure;
mod variables;
mod procedures;
mod preprocessor;
mod intermediate;

use intermediate::program_translator;
use lalrpop_util::lalrpop_mod;
use std::fs;
use std::str::FromStr;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

// Spytać czy błędy muszą wyświetlać linijkę

fn main() {
    let parser = grammar::program_allParser::new();
    // let file = std::fs::read_to_string("testy/error8.imp").unwrap();
    let file = std::fs::read_to_string("program2.imp").unwrap();
    let ret = parser.parse(&file).unwrap();
    // println!("{:?}", ret);
    let mut translator = program_translator::Translator::new();
    match translator.translate(ret) {
        Ok(_) => {
            println!("Compilation successful!");
        }
        Err(error) => {
            println!("Error happened during compilation:");
            println!("{:?}", error);
            return;
        }
    }
    println!();
    println!();
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
