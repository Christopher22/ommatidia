#[derive(Debug)]
pub struct Error {
    pub dataset: String,
    pub details: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    Io(String, std::io::Error),
    PatternInvalid(glob::PatternError),
    GlobIo(glob::GlobError),
    Aborted,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.details {
            ErrorType::Io(path, error) => write!(f, "loading sample '{}' failed: {}", path, error),
            ErrorType::PatternInvalid(error) => {
                write!(f, "invalid pattern for loading samples: {}", error)
            }
            ErrorType::GlobIo(error) => {
                write!(f, "querying samples given expression failed: {}", error)
            }
            ErrorType::Aborted => {
                write!(f, "loading and sending samples was aborted")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<glob::PatternError> for ErrorType {
    fn from(error: glob::PatternError) -> Self {
        Self::PatternInvalid(error)
    }
}

impl From<glob::GlobError> for ErrorType {
    fn from(error: glob::GlobError) -> Self {
        Self::GlobIo(error)
    }
}
