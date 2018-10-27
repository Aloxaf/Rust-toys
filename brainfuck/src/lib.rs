extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate podio;

mod parser;

pub use parser::{BrainFKParser, Rule};
use pest::Parser;
use pest::iterators::Pair;
use podio::ReadPodExt;
use std::{error, fmt};
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub struct Interpreter<R, W> {
    data: Vec<u8>,
    size: usize,
    istream: R,
    ostream: W,
    /// 当前指针位置
    pub index: usize,
}

#[derive(Debug)]
pub enum Error {
    /// 语法分析错误
    ParseError(pest::error::Error<Rule>),
    /// 指针越界
    StackPointerOutOfBoundary,
    /// IO错误
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Parse Error:\n{}", err),
            Error::StackPointerOutOfBoundary => write!(f, "Stack pointer out of boundary."),
            Error::IOError(ref err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseError(ref err) => err.description(),
            Error::StackPointerOutOfBoundary => "Stack pointer out of boundary.",
            Error::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IOError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl<R, W> Interpreter<R, W>
where
    R: Read,
    W: Write,
{
    /// 实例化一个解释器\
    ///
    /// # 参数
    ///
    /// `size` - 堆栈大小\
    /// `istream` - 输入流\
    /// `ostream` - 输出流
    pub fn new(size: usize, istream: R, ostream: W) -> Interpreter<R, W> {
        Interpreter {
            data: vec![0; size],
            index: 0,
            size,
            istream,
            ostream,
        }
    }

    /// 指针指向的cell数值+1
    fn incd(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_add(1);
    }

    /// 指针指向的cell数值-1
    fn decd(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_sub(1);
    }

    /// 指针+1
    fn incp(&mut self) -> Result<(), Error> {
        self.index += 1;
        if self.index < self.size {
            Ok(())
        } else {
            Err(Error::StackPointerOutOfBoundary)
        }
    }

    /// 指针-1
    fn decp(&mut self) -> Result<(), Error> {
        if self.index > 0 {
            self.index -= 1;
            Ok(())
        } else {
            Err(Error::StackPointerOutOfBoundary)
        }
    }

    /// 读取当前cell的数据
    fn read(&self) -> u8 {
        self.data[self.index]
    }

    /// 往当前cell写入数据
    fn write(&mut self, n: u8) {
        self.data[self.index] = n;
    }

    /// 执行一段代码
    pub fn eval(&mut self, code: &str) -> Result<(), Error> {
        fn parse(code: &str) -> Result<Pair<Rule>, pest::error::Error<Rule>> {
            match BrainFKParser::parse(Rule::code, &code.trim()) {
                Ok(mut tokens) => Ok(tokens.next().unwrap()),
                Err(e) => Err(e),
            }
        }
        let tokens = parse(code.trim()).map_err(Error::ParseError)?;
        self.eval_tokens(&tokens, false)
    }

    /// 运行 `Tokens`
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
