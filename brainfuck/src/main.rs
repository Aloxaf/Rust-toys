extern crate brainfuck;
extern crate pest;
extern crate podio;

use brainfuck::{BrainFKParser, Interpreter, Rule};
use pest::iterators::Pair;
use pest::Parser;
use std::error::Error;
use std::io::{self, Read};
use std::process;

fn main() {
    let mut interpreter = Interpreter::new(256, io::stdin(), io::stdout());
    interpreter.index = 100;

    let mut code = String::new();
    io::stdin().read_to_string(&mut code).unwrap();

    let tokens: Pair<Rule> = BrainFKParser::parse(Rule::code, &code.trim())
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        })
        .next()
        .unwrap();
    // +[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.
    interpreter
        .eval_tokens(&tokens, false)
        .unwrap_or_else(|e| {
            eprintln!("{}", e)
        });
}
