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
            } => write!(
                f,
                "{operation} failed: {domain} ({code}) — {description}"
            ),
        }
    }
}

impl std::error::Error for CoreWlanError {}
