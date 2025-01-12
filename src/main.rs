mod structure;
mod translator;
mod variables;
mod procedures;

use std::str::FromStr;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

fn main() {
    let parser = grammar::program_allParser::new();
    let file = std::fs::read_to_string("program3.imp").unwrap();
    let ret = parser.parse(&file).unwrap();
    i64::from_str("123").unwrap();
    println!("{:?}", ret);

}
