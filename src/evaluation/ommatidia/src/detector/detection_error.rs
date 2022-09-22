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
                    "the pupil detection algorithm '{}' responded unexpectedly: {}",
                    self.detector, reason
                )
            }
            DetectionErrorType::ConnectionFailure(error) => {
                write!(
                    f,
                    "connection failed tp detection algorithm '{}': {}",
                    self.detector, error
                )
            }
            DetectionErrorType::EstimationInvalid(identifier) => {
                write!(
                    f,
                    "pupil detection algorithm '{}' sent invalid payload during detection of sample '{}'",
                    self.detector,
                    identifier
                )
            }
            DetectionErrorType::EstimationResponseUnexpected(identifier, error) => {
                write!(
                    f,
                    "pupil detection algorithm '{}' returned an unexpected response for sample '{}': {}",
                    self.detector, identifier, error
                )
            }
            DetectionErrorType::EstimationFailed(identifier, message) => write!(
                f,
                "pupil detection algorithm '{} failed to estimate a pupil for sample '{}': {}",
                self.detector, identifier, message
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
