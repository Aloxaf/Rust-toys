extern crate brainfuck;
extern crate pest;
extern crate podio;

use brainfuck::Interpreter;
use std::io::{self, Read};

fn main() {
    let mut interpreter = Interpreter::new(256, io::stdin(), io::stdout());
    interpreter.index = 100;

    let mut code = String::new();
    io::stdin().read_to_string(&mut code).unwrap();

    interpreter.eval(&code.trim()).unwrap_or_else(|e| {
        eprintln!("{}", e);
    });

//    // +[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.
}
