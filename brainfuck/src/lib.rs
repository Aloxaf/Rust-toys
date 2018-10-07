extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "brainfuck.pest"]
pub struct BrainFKParser;

#[derive(Debug, Clone)]
pub struct Stack {
    data: Vec<u8>,
    size: usize,
    pub index: usize,
}

#[derive(Debug)]
pub enum Error {
    StackPointerOutOfBoundary,
}

impl Stack {
    pub fn new(size: usize) -> Stack {
        Stack {
            data: vec![0; size],
            size: size,
            index: 0,
        }
    }

    pub fn incd(&mut self) {
        self.data[self.index] += 1;
    }

    pub fn decd(&mut self) {
        self.data[self.index] -= 1;
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
}
