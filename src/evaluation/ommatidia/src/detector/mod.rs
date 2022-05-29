mod config;
mod connection;
mod detection;
mod detection_error;
mod detectors;
mod error;
mod name;

use std::{collections::HashMap, io::Read};

use bollard::{
    container::{
        Config as ContainerConfig, CreateContainerOptions, RemoveContainerOptions,
        StartContainerOptions, StopContainerOptions,
    },
    errors::Error as BollardError,
    models::{HostConfig, PortBinding},
};
use hyper::{Body, Method, StatusCode};

pub use self::{
    config::Config,
    connection::Connection,
    detection::Detection,
    detection_error::DetectionError,
    detectors::Detectors,
    error::{Error, ErrorType},
    name::{InvalidName, Name},
};
use super::{dataset::Samples, engine::Engine, estimate::Estimate, MetaData};

#[derive(Debug)]
pub struct Detector {
    name: Name,
    connection: Connection,
    pub meta_data: MetaData,
    engine: Engine,
    config: serde_json::Value,
}

impl Detector {
    pub async fn spawn<T: AsRef<str>>(
        name: Name,
        engine: Engine,
        image_name: T,
        config: serde_json::Value,
    ) -> Result<Detector, Error> {
        const TARGET_PORT: &str = "8080/tcp";
        let port = engine.get_free_port();

        // Create the container...
        engine
            .as_ref()
            .create_container(
                Some(CreateContainerOptions {
                    name: name.as_ref(),
                }),
                ContainerConfig {
                    image: Some(image_name.as_ref()),
                    exposed_ports: Some([(TARGET_PORT, HashMap::default())].into()),
                    host_config: Some(HostConfig {
                        port_bindings: Some(
                            [(
                                String::from(TARGET_PORT),
                                Some(vec![PortBinding {
                                    host_port: Some(port.to_string()),
                                    ..Default::default()
                                }]),
                            )]
                            .into(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await
            .map_err(|err| match err {
                BollardError::DockerResponseServerError {
                    status_code: 404,
                    message: _,
                } => Error {
                    detector: name.clone(),
                    details: ErrorType::ImageUnknown(image_name.as_ref().to_string()),
                },
                error => Error {
                    detector: name.clone(),
                    details: ErrorType::CreationFailed(error.to_string()),
                },
            })?;

        // .. start it, ...
        engine
            .as_ref()
            .start_container(name.as_ref(), None::<StartContainerOptions<&str>>)
            .await
            .map_err(|error| Error {
                detector: name.clone(),
                details: ErrorType::CreationFailed(error.to_string()),
            })?;

        // ... connect to the algorithm running there, ...
        let mut connection = connection::Connection::new(&engine, port)
            .await
            .map_err(|_| Error {
                detector: name.clone(),
                details: ErrorType::ConnectionFailed,
            })?;

        // ... and parse the meta data.
        let meta_data = MetaData::from_container(&mut connection)
            .await
            .map_err(|error| Error {
                detector: name.clone(),
                details: ErrorType::ImageInvalid(error),
            })?;

        Ok(Detector {
            name,
            connection,
            meta_data,
            engine,
            config,
        })
    }

    pub async fn detect(&mut self, mut samples: Samples) -> Result<Vec<Detection>, DetectionError> {
        const ERROR_MESSAGE_NO_UTF8: &str = "response is not valid UTF8";

        // Instantiate the detector and build the path it is accessible
        let config = serde_json::to_string(&self.config).expect("JSON value is always valid");
        let detector_path = match self
            .connection
            .send(Method::POST, "/detections/", Body::from(config))
            .await
        {
            Ok((StatusCode::OK, response)) => {
                let detector_id: u32 = serde_json::from_reader(response).map_err(|error| {
                    DetectionError::CreationResponseUnexpected(format!(
                        "parsing error as JSON failed: {}",
                        error
                    ))
                })?;
                Ok(format!("/detections/{}/", detector_id))
            }
            Ok((StatusCode::BAD_REQUEST, mut response_stream)) => {
                let mut response = String::with_capacity(16);
                response_stream.read_to_string(&mut response).map_err(|_| {
                    DetectionError::CreationResponseUnexpected(ERROR_MESSAGE_NO_UTF8.into())
                })?;
                Err(DetectionError::ConfigRejected(response))
            }
            Ok((unexpected_status, _)) => Err(DetectionError::CreationResponseUnexpected(format!(
                "response code '{}' unexpected",
                unexpected_status
            ))),
            Err(error) => Err(error.into()),
        }?;

        // Apply the algorithm on all samples
        let mut detections = Vec::with_capacity(32);
        while let Ok(sample) = samples.recv().await {
            detections.push(
                match self
                    .connection
                    .send(Method::POST, &detector_path, sample.content.into())
                    .await
                {
                    Ok((StatusCode::OK, response)) => {
                        let estimate: Estimate = serde_json::from_reader(response)
                            .or(Err(DetectionError::EstimationInvalid))?;
                        Detection::ok(sample.identifier, self.name.clone(), estimate)
                    }
                    Ok((StatusCode::BAD_REQUEST, mut response_stream)) => {
                        let mut failure_message = String::with_capacity(16);
                        response_stream
                            .read_to_string(&mut failure_message)
                            .map_err(|_| {
                                DetectionError::EstimationResponseUnexpected(
                                    ERROR_MESSAGE_NO_UTF8.into(),
                                )
                            })?;
                        Detection::failed(sample.identifier, self.name.clone(), failure_message)
                    }
                    Ok((unexpected_status, _)) => {
                        return Err(DetectionError::EstimationResponseUnexpected(format!(
                            "response code '{}' unexpected",
                            unexpected_status
                        )));
                    }
                    Err(error) => {
                        return Err(error.into());
                    }
                },
            );
        }

        Ok(detections)
    }

    pub async fn stop(self) -> bool {
        if self
            .engine
            .as_ref()
            .stop_container(self.name.as_ref(), Some(StopContainerOptions { t: 5 }))
            .await
            .is_err()
        {
            return false;
        }

        self.engine
            .as_ref()
            .remove_container(
                self.name.as_ref(),
                Some(RemoveContainerOptions {
                    v: true,
                    force: true,
                    link: true,
                }),
            )
            .await
            .is_err()
    }
}
