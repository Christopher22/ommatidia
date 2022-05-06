#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    PatternInvalid(glob::PatternError),
    GlobIo(glob::GlobError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "loading failed: {}", error),
            Error::PatternInvalid(error) => write!(f, "invalid pattern: {}", error),
            Error::GlobIo(error) => write!(f, "loading failed: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<glob::PatternError> for Error {
    fn from(error: glob::PatternError) -> Self {
        Self::PatternInvalid(error)
    }
}

impl From<glob::GlobError> for Error {
    fn from(error: glob::GlobError) -> Self {
        Self::GlobIo(error)
    }
}
