use crate::{detector::Name, MetaDataLoadingError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub detector: Name,
    pub details: ErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    MultipleNames,
    UnknownEngine(String),
    AmbiguesEngine,
    ImageUnknown(String),
    CreationFailed(String),
    ConnectionFailed,
    ImageInvalid(MetaDataLoadingError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let detector = self.detector.as_ref();
        match &self.details {
            ErrorType::MultipleNames => {
                write!(f, "at least two detector with name '{}' exists", detector)
            }
            ErrorType::UnknownEngine(engine) => write!(
                f,
                "an the engine '{}' for detector '{}' is not known",
                engine, detector
            ),
            ErrorType::AmbiguesEngine => write!(f, "unable to select a default engine for detector '{}' without explicit specification", self.detector.as_ref()),
            ErrorType::ImageUnknown(image_name) => {
                write!(f, "specified image '{}' for detector '{}' is invalid", image_name, detector)
            }
            ErrorType::CreationFailed(message) => {
                write!(f, "Docker failed to create and start image for detector '{}': {}", detector, message)
            }
            ErrorType::ConnectionFailed => {
                write!(f, "connecting to the started container of detector '{}' failed", detector)
            }
            ErrorType::ImageInvalid(error) => write!(
                f,
                "unable to communicate with the executable inside the container of '{}' as a pupil detector: {}",
                detector, error
            ),
        }
    }
}

impl std::error::Error for Error {}
