use failure::Error;
use regex;
use reqwest;
use std::fmt;
use std::option::NoneError;

#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "NoneValue: ")]
    NoneValue,
    #[fail(display = "NornalError: ")]
    NormalError,
}

impl From<NoneError> for MyError {
    fn from(_err: NoneError) -> Self {
        MyError::NoneValue
    }
}

impl From<Error> for MyError {
    fn from(_err: Error) -> Self {
        MyError::NormalError
    }
}

impl From<regex::Error> for MyError {
    fn from(_err: regex::Error) -> Self {
        MyError::NormalError
    }
}

impl From<reqwest::Error> for MyError {
    fn from(_err: reqwest::Error) -> Self {
        MyError::NormalError
    }
}


#[derive(Fail, Debug)]
pub enum GetAcidError {
    NetWorkError,
    CannotFindError,
    AlreadyLogin,
}

impl fmt::Display for GetAcidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAcidError::NetWorkError => write!(f, "Network error!"),
            GetAcidError::CannotFindError => write!(f, "Cannot detect acid!"),
            GetAcidError::AlreadyLogin => write!(f, "Already login!"),
        }
    }
}

impl From<NoneError> for GetAcidError {
    fn from(_err: NoneError) -> Self {
        GetAcidError::CannotFindError
    }
}

impl From<regex::Error> for GetAcidError {
    fn from(_err: regex::Error) -> Self {
        GetAcidError::CannotFindError
    }
}

impl From<reqwest::Error> for GetAcidError {
    fn from(_err: reqwest::Error) -> Self {
        GetAcidError::NetWorkError
    }
}


//
//
//pub enum ErrorKind {
//    AuthFailed(String),
//    GetAcidError(String),
//    NetworkError(String),
//}
//
//impl fmt::Display for ErrorKind {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            ErrorKind::AuthFailed(ref c) => write!(f, "{}", c),
//            ErrorKind::GetAcidError(ref c) => write!(f, "{}", c),
//            ErrorKind::NetworkError(ref c) => write!(f, "{}", c),
//        }
//    }
//}