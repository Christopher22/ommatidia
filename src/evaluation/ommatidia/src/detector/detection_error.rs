use crate::dataset::Identifier;

use super::Name;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectionError {
    pub detector: Name,
    pub error_type: DetectionErrorType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetectionErrorType {
    ConfigRejected(String),
    CreationResponseUnexpected(String),
    ConnectionFailure(String),
    EstimationInvalid(Identifier),
    EstimationResponseUnexpected(Identifier, String),
    EstimationFailed(Identifier, String),
}

impl std::fmt::Display for DetectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            DetectionErrorType::ConfigRejected(reason) => write!(f, "invalid config: {}", reason),
            DetectionErrorType::CreationResponseUnexpected(reason) => {
                write!(
                    f,
                    "the pupil detection algorithm responded unexpectedly: {}",
                    reason
                )
            }
            DetectionErrorType::ConnectionFailure(error) => {
                write!(f, "connection failed: {}", error)
            }
            DetectionErrorType::EstimationInvalid(identifier) => {
                write!(
                    f,
                    "pupil detection algorithm '{}' sent invalid payload during detection",
                    identifier
                )
            }
            DetectionErrorType::EstimationResponseUnexpected(identifier, error) => {
                write!(
                    f,
                    "pupil detection algorithm '{}' returned an unexpected response: {}",
                    identifier, error
                )
            }
            DetectionErrorType::EstimationFailed(identifier, message) => write!(
                f,
                "pupil detection algorithm '{} failed to estimate a pupil: {}",
                identifier, message
            ),
        }
    }
}

impl std::error::Error for DetectionError {}

impl From<hyper::Error> for DetectionErrorType {
    fn from(error: hyper::Error) -> Self {
        DetectionErrorType::ConnectionFailure(error.to_string())
    }
}
