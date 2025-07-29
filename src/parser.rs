use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/pascal.pest"]
pub struct PascalParser;

pub use pest::iterators::Pairs;
pub use pest::Parser as PestParser;

pub fn parse_program(input: &str) {
    match PascalParser::parse(Rule::program, input) {
        Ok(pairs) => {
            println!("Parse success!");
            for pair in pairs {
                println!("{:?}", pair);
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}

