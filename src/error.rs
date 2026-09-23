//! Errors returned by the `corewlan` crate.

use core::fmt;

pub type Result<T> = std::result::Result<T, CoreWlanError>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CoreWlanError {
    UnexpectedNull(&'static str),
    OperationFailed(&'static str),
    ObjectiveCError {
        operation: &'static str,
        code: isize,
        domain: String,
        description: String,
    },
    OsStatusError {
        operation: &'static str,
        status: i32,
    },
    InteriorNul(&'static str),
}

impl fmt::Display for CoreWlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedNull(what) => write!(f, "{what} returned NULL"),
            Self::OperationFailed(op) => write!(f, "{op} failed"),
            Self::ObjectiveCError {
                operation,
                code,
                domain,
                description,
            } => write!(f, "{operation} failed: {domain} ({code}) — {description}"),
            Self::OsStatusError { operation, status } => {
                write!(f, "{operation} failed with OSStatus {status}")
            }
            Self::InteriorNul(argument) => {
                write!(f, "{argument} contains an interior NUL byte")
            }
        }
    }
}

impl std::error::Error for CoreWlanError {}
