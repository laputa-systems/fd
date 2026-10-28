//! Typed CLI errors. Rendered to stderr with a nonzero exit.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliError {
    UnknownOption(String),
    UnknownShort(char),
    MissingValue { option: &'static str },
    InvalidValue { option: &'static str, value: String, reason: String },
    Conflict { first: String, second: String },
    Requires { option: &'static str, requires: &'static str },
    NonUtf8 { option: &'static str },
    EmptyExec { option: &'static str },
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOption(name) => write!(f, "error: unexpected argument '--{name}' found"),
            Self::UnknownShort(c) => write!(f, "error: unexpected argument '-{c}' found"),
            Self::MissingValue { option } => {
                write!(f, "error: a value is required for '--{option}' but none was supplied")
            }
            Self::InvalidValue { option, value, reason } => write!(
                f,
                "error: invalid value '{value}' for '--{option}': {reason}"
            ),
            Self::Conflict { first, second } => write!(
                f,
                "error: the argument '{first}' cannot be used with '{second}'"
            ),
            Self::Requires { option, requires } => write!(
                f,
                "error: the argument '--{option}' requires '--{requires}'"
            ),
            Self::NonUtf8 { option } => write!(
                f,
                "error: invalid UTF-8 for '--{option}': value must be valid UTF-8"
            ),
            Self::EmptyExec { option } => {
                write!(f, "error: a value is required for '--{option}' but none was supplied")
            }
        }
    }
}

impl std::error::Error for CliError {}
