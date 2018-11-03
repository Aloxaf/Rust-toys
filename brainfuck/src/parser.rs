use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "brainfuck.pest"]
pub struct BrainFKParser;
