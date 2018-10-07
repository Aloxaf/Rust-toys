extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate podio;

mod parser;

pub use parser::{BrainFKParser, Rule};
use pest::iterators::Pair;
use podio::ReadPodExt;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub struct Interpreter<R, W> {
    data: Vec<u8>,
    size: usize,
    istream: R,
    ostream: W,
    pub index: usize,
}

#[derive(Debug)]
pub enum Error {
    StackPointerOutOfBoundary,
    IOError(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(format!("IO Error: {}", e))
    }
}

impl<R, W> Interpreter<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(size: usize, istream: R, ostream: W) -> Interpreter<R, W> {
        Interpreter {
            data: vec![0; size],
            index: 0,
            size,
            istream,
            ostream,
        }
    }

    pub fn incd(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_add(1);
    }

    pub fn decd(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_sub(1);
    }

    pub fn incp(&mut self) -> Result<(), Error> {
        self.index += 1;
        if self.index < self.size {
            Ok(())
        } else {
            Err(Error::StackPointerOutOfBoundary)
        }
    }

    pub fn decp(&mut self) -> Result<(), Error> {
        if self.index > 0 {
            self.index -= 1;
            Ok(())
        } else {
            Err(Error::StackPointerOutOfBoundary)
        }
    }

    pub fn read(&self) -> u8 {
        self.data[self.index]
    }

    pub fn write(&mut self, n: u8) {
        self.data[self.index] = n;
    }

    pub fn eval_tokens(&mut self, tokens: &Pair<Rule>, _loop: bool) -> Result<(), Error> {
        while !(_loop && self.read() == 0) {
            for op in tokens.clone().into_inner() {
                match op.as_rule() {
                    Rule::incp => self.incp()?,
                    Rule::decp => self.decp()?,
                    Rule::incd => self.incd(),
                    Rule::decd => self.decd(),
                    Rule::accept => {
                        let input = self.istream.read_u8()?;
                        self.write(input);
                    }
                    Rule::output => {
                        let output = self.read() as char;
                        write!(self.ostream, "{}", output)?;
                    }
                    Rule::loop_body => self.eval_tokens(&op, true)?,
                    _ => unreachable!(),
                }
                self.ostream.flush()?;
            }
            if !(_loop && self.read() != 0) {
                break;
            }
        }
        Ok(())
    }
}
