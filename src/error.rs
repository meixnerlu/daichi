#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorType,
    reason: String,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Database,
    Discord,
    Any,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<poise::serenity_prelude::Error> for Error {
    fn from(value: poise::serenity_prelude::Error) -> Self {
        Self {
            kind: ErrorType::Discord,
            reason: value.to_string(),
        }
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Self {
            kind: ErrorType::Database,
            reason: value.to_string(),
        }
    }
}

impl Error {
    pub fn from_any(error: impl std::error::Error) -> Self {
        Self {
            kind: ErrorType::Any,
            reason: error.to_string(),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            kind: ErrorType::Any,
            reason: value.to_string(),
        }
    }
}
