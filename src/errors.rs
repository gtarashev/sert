//      imports
//      =======
use std::fmt::{self, Debug, Display};

//      structures
//      ==========
// Used when parsing command line arguments
pub enum EnvironmentParseError {
    NullArg(String),
    InvalidArg(String),
    InvalidPath(String),
    NotADir(String),
    InvalidAddr(String),
    InvalidPort(String),
    InvalidTimeout(String),
}

// Used when parsing an incoming request into a Request object
pub enum RequestParseError {
    InvalidMethodError(String),
    EmptyRequestError,
    InvalidRequestHeader,
}

//      impl(s)
//      =======
impl Display for EnvironmentParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NullArg(x) => write!(
                f,
                "Option {} expects an additional argument, but none was provided",
                x
            ),
            Self::InvalidArg(x) => write!(f, "Invalid option: {}", x),
            Self::InvalidPath(x) => write!(f, "Invalid directory: {}", x),
            Self::NotADir(x) => write!(f, "Expected a directory: {}", x),
            Self::InvalidAddr(x) => write!(f, "Not a valid address: {}", x),
            Self::InvalidPort(x) => write!(f, "Not a valid port: {}", x),
            Self::InvalidTimeout(x) => write!(f, "Not a valid timeout: {}", x),
        }
    }
}

// --------
impl Display for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestParseError::InvalidMethodError(x) => write!(f, "Invalid request method: {}", x),
            RequestParseError::EmptyRequestError => write!(f, "Request was empty"),
            RequestParseError::InvalidRequestHeader => {
                write!(f, "Request header was not formatted correctly")
            }
        }
    }
}

impl Debug for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestParseError::InvalidMethodError(x) => write!(f, "Invalid request method: {}", x),
            RequestParseError::EmptyRequestError => write!(f, "Request was empty"),
            RequestParseError::InvalidRequestHeader => {
                write!(f, "Request header was not formatted correctly")
            }
        }
    }
}
