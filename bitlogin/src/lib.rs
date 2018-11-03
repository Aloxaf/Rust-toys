#![feature(try_trait)]
mod errors;
mod user;
mod utils;

pub use crate::errors::{GetAcidError, MyError};
pub use crate::user::User;
