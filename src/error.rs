use std::fmt;

#[derive(Debug)]
pub enum FlattenError {
    Io(std::io::Error),
    Pattern(String),
    Processing(String),
}

impl fmt::Display for FlattenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlattenError::Io(err) => write!(f, "IO error: {}", err),
            FlattenError::Pattern(msg) => write!(f, "Pattern error: {}", msg),
            FlattenError::Processing(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for FlattenError {}

impl From<std::io::Error> for FlattenError {
    fn from(err: std::io::Error) -> Self {
        FlattenError::Io(err)
    }
}

impl From<glob::PatternError> for FlattenError {
    fn from(err: glob::PatternError) -> Self {
        FlattenError::Pattern(err.to_string())
    }
}
