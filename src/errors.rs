use std::fmt::{self, Debug, Display};

pub enum RequestParseError {
    InvalidMethodError(String),
    EmptyRequestError,
    InvalidRequestHeader,
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
