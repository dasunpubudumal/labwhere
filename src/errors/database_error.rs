use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Database errors
pub struct ConnectivityError {
    pub message: String,
}

impl Display for ConnectivityError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Debug for ConnectivityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Error for ConnectivityError {}
