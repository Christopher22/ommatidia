use crate::MetaDataLoadingError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreationError {
    ImageUnknown(String),
    CreationFailed(String),
    ConnectionFailed,
    ImageInvalid(MetaDataLoadingError),
}

impl std::fmt::Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CreationError::ImageUnknown(image_name) => {
                write!(f, "specified image '{}' is invalid", image_name)
            }
            CreationError::CreationFailed(message) => {
                write!(f, "Docker failed to create and start image: {}", message)
            }
            CreationError::ConnectionFailed => {
                f.write_str("connecting to the started container failed")
            }
            CreationError::ImageInvalid(error) => write!(
                f,
                "unable to communicate with the container as a pupil detector: {}",
                error
            ),
        }
    }
}

impl std::error::Error for CreationError {}
