use std::io::Read;

use hyper::{Body, Method, StatusCode};
use serde::{Deserialize, Serialize};

use super::detector::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputType {
    Point,
    Ellipse,
    Mask,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaData {
    name: String,
    additional_information: String,
    authors: Vec<String>,
    license: String,
    prediction: OutputType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConnectionFailed,
    RequestFailed(StatusCode, String),
    InvalidPayload(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::ConnectionFailed => f.write_str("communication with the container failed"),
            Error::RequestFailed(status_code, error) => write!(
                f,
                "request failed with HTTP code {}: {}",
                status_code, error
            ),
            Error::InvalidPayload(error) => write!(f, "response is not valid meta data: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl MetaData {
    pub async fn from_container(connection: &mut Connection) -> Result<Self, Error> {
        let response = match connection.send(Method::GET, "/", Body::empty()).await {
            Ok((StatusCode::OK, response)) => response,
            Ok((status_code, mut response)) => {
                let mut error = String::new();
                if response.read_to_string(&mut error).is_err() {
                    error.push_str("<Invalid error message>");
                }
                return Err(Error::RequestFailed(status_code, error));
            }
            Err(_) => return Err(Error::ConnectionFailed),
        };
        serde_json::from_reader(response).map_err(|error| Error::InvalidPayload(error.to_string()))
    }
}
