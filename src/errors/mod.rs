pub mod database_error;
pub mod name_format_error;
pub mod not_found_error;

use std::error::Error;
use std::fmt::Debug;

use crate::errors::database_error::ConnectivityError;
use crate::errors::not_found_error::NotFoundError;

/// A generalised error for Labware
#[derive(Debug)]
pub enum LabwhereError {
    NotFound(NotFoundError),
    ConnectivityError(ConnectivityError),
}

impl std::fmt::Display for LabwhereError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabwhereError::NotFound(err) => write!(f, "{}", err),
            LabwhereError::ConnectivityError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for LabwhereError {}

impl From<NotFoundError> for LabwhereError {
    fn from(err: NotFoundError) -> Self {
        LabwhereError::NotFound(err)
    }
}

impl From<ConnectivityError> for LabwhereError {
    fn from(err: ConnectivityError) -> Self {
        LabwhereError::ConnectivityError(err)
    }
}
