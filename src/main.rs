extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

fn main() {
    let my_file = fs::read_to_string("test.obo").unwrap();
    let pairs = IdentParser::parse(Rule::programa, &my_file).unwrap_or_else(|e| panic!("{}", e));
    println!("{:#?}", pairs);
    
}