mod creation_error;
mod detection;
mod detection_error;

use std::{collections::HashMap, io::Read};

use bollard::{
    container::{
        Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
        StopContainerOptions,
    },
    errors::Error as BollardError,
    models::{HostConfig, PortBinding},
};
use hyper::{Body, Method, StatusCode};

pub use self::{
    creation_error::CreationError, detection::Detection, detection_error::DetectionError,
};
use super::{connection::Connection, estimate::Estimate, files::Samples, Engine, MetaData};

#[derive(Debug)]
pub struct Detector {
    name: String,
    connection: Connection,
    pub meta_data: MetaData,
    engine: Engine,
}

impl Detector {
    pub async fn spawn<T: AsRef<str>>(
        engine: Engine,
        image_name: T,
    ) -> Result<Detector, CreationError> {
        const TARGET_PORT: &str = "8080/tcp";
        let name = uuid::Uuid::new_v4().to_string();
        let port = engine.get_free_port();

        // Create the container...
        engine
            .as_ref()
            .create_container(
                Some(CreateContainerOptions {
                    name: name.as_str(),
                }),
                Config {
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
                } => CreationError::ImageUnknown(image_name.as_ref().to_string()),
                error => CreationError::CreationFailed(error.to_string()),
            })?;

        // .. start it, ...
        engine
            .as_ref()
            .start_container(name.as_str(), None::<StartContainerOptions<&str>>)
            .await
            .map_err(|error| CreationError::CreationFailed(error.to_string()))?;

        // ... connect to the algorithm running there, ...
        let mut connection = Connection::new(&engine, port)
            .await
            .or(Err(CreationError::ConnectionFailed))?;

        // ... and parse the meta data.
        let meta_data = MetaData::from_container(&mut connection)
            .await
            .map_err(CreationError::ImageInvalid)?;

        Ok(Detector {
            name,
            connection,
            meta_data,
            engine,
        })
    }

    pub async fn detect<'a, T: serde::Serialize>(
        &mut self,
        mut samples: Samples,
        config: &T,
    ) -> Result<Vec<Detection>, DetectionError> {
        const ERROR_MESSAGE_NO_UTF8: &str = "response is not valid UTF8";

        // Instantiate the detector and build the path it is accessible
        let config = serde_json::to_string(config).or(Err(DetectionError::ConfigInvalid))?;
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
                        Detection::ok(sample.identifier, estimate)
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
                        Detection::failed(sample.identifier, failure_message)
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
            .stop_container(&self.name, Some(StopContainerOptions { t: 5 }))
            .await
            .is_err()
        {
            return false;
        }

        self.engine
            .as_ref()
            .remove_container(
                &self.name,
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
