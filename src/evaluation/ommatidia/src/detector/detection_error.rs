#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetectionError {
    ConfigRejected(String),
    CreationResponseUnexpected(String),
    EstimationInvalid,
    EstimationResponseUnexpected(String),
    ConnectionFailure(String),
}

impl std::fmt::Display for DetectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectionError::ConfigRejected(reason) => write!(f, "invalid config: {}", reason),
            DetectionError::CreationResponseUnexpected(reason) => {
                write!(
                    f,
                    "the pupil detection algorithm responded unexpectedly: {}",
                    reason
                )
            }
            DetectionError::ConnectionFailure(error) => write!(f, "connection failed: {}", error),
            DetectionError::EstimationInvalid => {
                f.write_str("algorithm sent invalid payload during detection")
            }
            DetectionError::EstimationResponseUnexpected(error) => {
                write!(
                    f,
                    "the pupil detection algorithm returned an unexpected estimate: {}",
                    error
                )
            }
        }
    }
}

impl std::error::Error for DetectionError {}

impl From<hyper::Error> for DetectionError {
    fn from(error: hyper::Error) -> Self {
        DetectionError::ConnectionFailure(error.to_string())
    }
}
