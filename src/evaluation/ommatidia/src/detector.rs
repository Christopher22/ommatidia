use std::collections::HashMap;

use bollard::{
    container::{
        Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
        StopContainerOptions,
    },
    errors::Error as BollardError,
    models::{HostConfig, PortBinding},
};

pub use super::{
    connection::Connection, Engine, MetaData, MetaDataLoadingError, RemoteConfig, SslConfig,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ImageUnknown(String),
    CreationFailed(String),
    ConnectionFailed,
    ImageInvalid(MetaDataLoadingError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::ImageUnknown(image_name) => {
                write!(f, "specified image '{}' is invalid", image_name)
            }
            Error::CreationFailed(message) => {
                write!(f, "Docker failed to create and start image: {}", message)
            }
            Error::ConnectionFailed => f.write_str("connecting to the started container failed"),
            Error::ImageInvalid(error) => write!(
                f,
                "unable to communicate with the container as a pupil detector: {}",
                error
            ),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Detector {
    name: String,
    connection: Connection,
    pub meta_data: MetaData,
    engine: Engine,
}

impl Detector {
    pub async fn spawn<T: AsRef<str>>(engine: Engine, image_name: T) -> Result<Detector, Error> {
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
                } => Error::ImageUnknown(image_name.as_ref().to_string()),
                error => Error::CreationFailed(error.to_string()),
            })?;

        // .. start it, ...
        engine
            .as_ref()
            .start_container(name.as_str(), None::<StartContainerOptions<&str>>)
            .await
            .map_err(|error| Error::CreationFailed(error.to_string()))?;

        // ... connect to the algorithm running there, ...
        let mut connection = Connection::new(&engine, port)
            .await
            .or(Err(Error::ConnectionFailed))?;

        // ... and parse the meta data.
        let meta_data = MetaData::from_container(&mut connection)
            .await
            .map_err(Error::ImageInvalid)?;

        Ok(Detector {
            name,
            connection,
            meta_data,
            engine,
        })
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
