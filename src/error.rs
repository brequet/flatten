use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlattenError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Pattern error: {0}")]
    Pattern(String),

    #[error("Processing error: {0}")]
    Processing(String),
}

impl From<glob::PatternError> for FlattenError {
    fn from(err: glob::PatternError) -> Self {
        FlattenError::Pattern(err.to_string())
    }
}
