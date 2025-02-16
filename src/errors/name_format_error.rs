use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Error struct for containing name formatting errors
pub struct NameFormatError {
    /// Message contained within the exception
    pub message: String,
}

impl Display for NameFormatError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Debug for NameFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl Error for NameFormatError {}
