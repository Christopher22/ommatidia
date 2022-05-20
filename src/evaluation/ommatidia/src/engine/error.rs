use bollard::errors::Error as EngineError;

#[derive(Debug)]
pub enum Error {
    ConnectionFailed(Option<String>, EngineError),
    DuplicatedName(String),
    DuplicatedHost(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionFailed(None, error) => write!(
                f,
                "connecting to the local Docker instance (as default given missing explicit specification) failed: {}",
                error
            ),
            Error::ConnectionFailed(Some(name), error) => write!(
                f,
                "connecting engine '{}' to Docker failed: {}",
                name,
                error
            ),
            Error::DuplicatedName(name) => {
                write!(f, "two Docker hosts share the same name '{}'", name)
            }
            Error::DuplicatedHost(name) => {
                write!(
                    f,
                    "two Docker hosts share the same remote host specification '{}'",
                    name
                )
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<EngineError> for Error {
    fn from(error: EngineError) -> Self {
        Error::ConnectionFailed(None, error)
    }
}
