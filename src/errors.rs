use std::fmt::{self, Debug, Display};

pub struct MethodError {
    given_method: String,
}

impl Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid request method: {}", self.given_method)
    }
}

impl Debug for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid request method: {}", self.given_method)
    }
}

impl MethodError {
    pub fn new(given_method: String) -> Self {
        Self { given_method }
    }
}
