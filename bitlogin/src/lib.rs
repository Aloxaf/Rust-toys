#![feature(try_trait)]
#[macro_use] extern crate failure;
#[macro_use]
extern crate json;
extern crate nix;
extern crate regex;
extern crate reqwest;
extern crate sha1;

mod utils;
mod user;
mod errors;

pub use self::user::User;
pub use self::errors::{MyError, GetAcidError};
