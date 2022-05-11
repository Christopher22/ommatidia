mod config;
mod engines;
mod readable_file_path;
mod remote_config;

use std::rc::Rc;

use bollard::{errors::Error, Docker};

pub use self::{
    config::Config,
    engines::{Engines, Error as EnginesError},
    readable_file_path::ReadableFilePath,
    remote_config::{RemoteConfig, SslConfig},
};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(try_from = "Config")]
pub struct Engine {
    backend: Rc<Docker>,
    config: Rc<Config>,
}

impl Engine {
    const CONNECTION_TIMEOUT_SECONDS: u64 = 16;

    pub fn local<T: AsRef<str>>(name: T) -> Result<Engine, Error> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Engine {
            backend: docker.into(),
            config: Rc::new(Config {
                name: name.as_ref().to_owned(),
                remote_config: None,
            }),
        })
    }

    pub fn host(&self) -> &str {
        self.config
            .remote_config
            .as_ref()
            .map(|host| host.host.as_str())
            .unwrap_or("127.0.0.1")
    }

    pub fn get_free_port(&self) -> u16 {
        // ToDo: What to do on remote server? Just guess, by now...
        portpicker::pick_unused_port().expect("free port")
    }
}

impl AsRef<Docker> for Engine {
    fn as_ref(&self) -> &Docker {
        &self.backend
    }
}

impl TryFrom<Config> for Engine {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        match config.remote_config.as_ref() {
            Some(remote_config) => {
                let addr = remote_config.to_string();
                let docker = match &remote_config.authentication {
                    Some(ssl) => Docker::connect_with_ssl(
                        &addr,
                        ssl.private_key.as_ref(),
                        ssl.cert_key.as_ref(),
                        ssl.certificate_chain.as_ref(),
                        Engine::CONNECTION_TIMEOUT_SECONDS,
                        bollard::API_DEFAULT_VERSION,
                    ),
                    None => Docker::connect_with_http(
                        &addr,
                        Engine::CONNECTION_TIMEOUT_SECONDS,
                        bollard::API_DEFAULT_VERSION,
                    ),
                }?;

                Ok(Engine {
                    backend: docker.into(),
                    config: config.into(),
                })
            }
            None => Engine::local(config.name),
        }
    }
}
