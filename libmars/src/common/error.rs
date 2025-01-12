use core::result::Result as CoreResult;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::common::x11::atoms::X11Atom;

pub type Result<T> = CoreResult<T, MarsError>;

#[derive(Debug,Clone)]
pub struct MarsError {
    kind: MarsErrorKind,
    info: Option<String>,
}

#[derive(Debug,Copy,Clone)]
pub enum MarsErrorKind {
    ConnectionFailed,
    FailedRequest,
    IllegalValue,
    InvalidInput,
    PropertyUnavailable,
    Unknown,
    UnsupportedProtocol,
}


impl MarsError {
    pub fn failed_request(call: &str) -> MarsError {
        MarsError {
            kind: MarsErrorKind::FailedRequest,
            info: Some(format!("{} failed", call)),
        }
    }

    pub fn failed_conversion(value: impl std::fmt::Debug, from: &str, to: &str) -> MarsError {
        MarsError {
            kind: MarsErrorKind::IllegalValue,
            info: Some(format!("unable to convert {:?} from {} to {}", value, from, to)),
        }
    }

    pub fn invalid_input(msg: impl ToString) -> MarsError {
        MarsError {
            kind: MarsErrorKind::InvalidInput,
            info: Some(msg.to_string()),
        }
    }

    pub fn invalid_response(request: impl Display) -> MarsError {
        MarsError {
            kind: MarsErrorKind::IllegalValue,
            info: Some(format!("invalid response value to request '{}'", request)),
        }
    }

    pub fn property_unavailable(property: impl ToString) -> MarsError {
        MarsError {
            kind: MarsErrorKind::PropertyUnavailable,
            info: Some(property.to_string()),
        }
    }

    pub fn unknown(info: impl ToString) -> MarsError {
        MarsError {
            kind: MarsErrorKind::Unknown,
            info: Some(info.to_string()),
        }
    }

    pub fn x11_unsupported_atom(atom: X11Atom) -> MarsError {
        MarsError {
            kind: MarsErrorKind::UnsupportedProtocol,
            info: Some(atom.to_string()),
        }
    }

    pub fn x11_open_display() -> MarsError {
        MarsError {
            kind: MarsErrorKind::ConnectionFailed,
            info: Some("XOpenDisplay".to_owned()),
        }
    }
}


impl Display for MarsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.kind.fmt(f)?;
        if let Some(info) = &self.info {
            write!(f, " ({})", info)?;
        }
        Ok(())
    }
}

impl From<MarsError> for String {
    fn from(value: MarsError) -> Self {
        value.to_string()
    }
}

impl Display for MarsErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let name = match self {
            Self::ConnectionFailed => "Connection failed",
            Self::FailedRequest => "Failed request",
            Self::IllegalValue => "Illegal value",
            Self::InvalidInput => "Invalid input",
            Self::PropertyUnavailable => "Property not available",
            Self::Unknown => "Unknown error",
            Self::UnsupportedProtocol => "Protocol not supported",
        };
        write!(f, "{}", name)
    }
}


