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
    let file = fs::read_to_string("program0.imp").unwrap();
    // let file = fs::read_to_string("testy/error1.imp").unwrap();
    // let file = fs::read_to_string("myprogram.imp").unwrap();
    let ret = parser.parse(&file).unwrap();
    let mut translator = program_translator::Translator::new();
    if let Some(program) = translator.compile(ret) {
        fs::write("output.mr", program).unwrap();
    } else {
        println!("Didnt write to file");
    }
    // translator.program.print();

}

#[test]
fn test(){
    let num = -15;
    let val = num >> 1;
    println!("{} ", val);
    let res = (val << 1)-num;
    println!("{}", res);
}
