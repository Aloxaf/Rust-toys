//! C++ like iostream, mainly ">>" and "<<" support
//! # Examples
//!
//! ## basic usage
//! ```
//! let mut name = String::new();
//! let mut age = 0;
//! cin >> &mut name >> &mut age;
//! cout << name << " is " << age << " years old." << endl;
//! ```
//!
//! ## read to eof
//! ```
//! let mut sum = 0;
//! let mut n = 0 ;
//! while cin >> &mut n != Eof {
//!     sum += n;
//! }
//! cout << "sum: " << sum << endl;
//! ```
use std::fmt::{self, Debug, Display, Formatter};
use std::io;
use std::io::prelude::*;
use std::ops;
use std::str::FromStr;

/// end of line
#[allow(non_camel_case_types)]
pub struct endl;
/// console input
#[derive(Clone, Copy, PartialEq)]
pub enum IStream {
    Success,
    TypeNotMatch,
    Eof,
}
/// console output
#[allow(non_camel_case_types)]
pub struct cout;

#[allow(non_upper_case_globals)]
pub const cin: IStream = IStream::Success;

pub use crate::IStream::Eof;

impl Display for endl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\n")
    }
}

impl<T> ops::Shl<T> for cout
where
    T: Display,
{
    type Output = Self;

    fn shl(self, output: T) -> Self::Output {
        print!("{}", output);
        self
    }
}

impl<T> ops::Shr<&mut T> for IStream
where
    T: FromStr,
    T::Err: Debug,
{
    type Output = Self;

    fn shr(self, input: &mut T) -> Self::Output {
        let mut ret = IStream::TypeNotMatch;
        while ret == IStream::TypeNotMatch {
            ret = match self {
                IStream::Success | IStream::TypeNotMatch => {
                    let str = io::stdin()
                        .bytes()
                        .map(|b| b.unwrap() as char)
                        .take_while(|&c| c != ' ' && c != '\n' && c != '\t')
                        .collect::<String>();
                    match str.trim().parse::<T>() {
                        Ok(value) => {
                            *input = value;
                            IStream::Success
                        }
                        Err(_err) => {
                            if str.trim() == "" {
                                IStream::Eof
                            } else {
                                IStream::TypeNotMatch
                            }
                        }
                    }
                }
                IStream::Eof => IStream::Eof,
            };
        }
        ret
    }
}

//#[cfg(test)]
//mod tests {
//    use super::{Cin, cout, endl};
//
//    #[test]
//    fn it_works() {
//        let mut n = 0;
//        let mut s = String::new();
//        cin >> &mut n >> &mut s;
//        cout << n << " " << s << endl;
//        cout << "123" << ' ' << 1 << endl;
//    }
//}
