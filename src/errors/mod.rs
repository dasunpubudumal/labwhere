use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct NotFoundError {
    pub message: String,
}

impl Display for NotFoundError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Debug for NotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Error for NotFoundError {}
