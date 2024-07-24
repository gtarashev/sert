use std::fmt::{self, Debug, Display};

pub enum RequestParseError {
    InvalidMethodError(String),
    EmptyRequestError,
    InvalidRequestHeader,
}

pub enum EnvironmentParseError {
    NullArg(String),
    InvalidArg(String),
    InvalidPath(String),
}

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
        }
    }
}

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
