extern crate brainfuck;
extern crate pest;
extern crate podio;

use brainfuck::{BrainFKParser, Rule, Stack};
use pest::iterators::Pair;
use pest::Parser;
use podio::ReadPodExt;
use std::io::{self, Write, Read};

fn eval(stack: &mut Stack, tokens: Pair<Rule>, _loop: bool) {
    loop {
        if _loop && stack.read() == 0 {
            break;
        }
        for op in tokens.clone().into_inner() {
            match op.as_rule() {
                Rule::incp => stack.incp().unwrap(),
                Rule::decp => stack.decp().unwrap(),
                Rule::incd => stack.incd(),
                Rule::decd => stack.decd(),
                Rule::accept => stack.write(io::stdin().read_u8().unwrap()),
                Rule::output => print!("{}", stack.read() as char),
                Rule::loop_body => eval(stack, op, true),
                _ => unreachable!(),
            }
            io::stdout().flush().unwrap();
        }
        if !_loop || (_loop && stack.read() == 0) {
            break;
        }
    }
}

fn main() {
    let mut stack = Stack::new(256);
    stack.index = 100;

    let mut code = String::new();
    io::stdin().read_to_string(&mut code).unwrap();

    let tokens: Pair<Rule> = BrainFKParser::parse(Rule::code, &code.trim()).unwrap().next().unwrap();
    // +[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.
    eval(&mut stack, tokens, false);
}
