use std::{path::PathBuf, rc::Rc};

use bollard::{errors::Error, Docker};

#[derive(Debug, Clone)]
pub struct Engine {
    backend: Rc<Docker>,
    remote_host: Option<RemoteConfig>,
}

impl Engine {
    pub fn host(&self) -> &str {
        self.remote_host
            .as_ref()
            .map(|host| host.host())
            .unwrap_or("127.0.0.0")
    }
}

impl AsRef<Docker> for Engine {
    fn as_ref(&self) -> &Docker {
        &self.backend
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RemoteConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub security: Option<SslConfig>,
}

impl RemoteConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("host.docker.internal")
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(2375)
    }
}

impl std::fmt::Display for RemoteConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host(), self.port())
    }
}

impl Engine {
    const CONNECTION_TIMEOUT_SECONDS: u64 = 16;

    pub fn connect_local() -> Result<Engine, Error> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Engine {
            backend: docker.into(),
            remote_host: None,
        })
    }

    pub fn get_free_port(&self) -> u16 {
        // ToDo: What to do on remote server? Just guess, by now...
        portpicker::pick_unused_port().expect("free port")
    }
}

impl TryFrom<RemoteConfig> for Engine {
    type Error = Error;

    fn try_from(config: RemoteConfig) -> Result<Self, Self::Error> {
        let addr = config.to_string();
        let docker = match &config.security {
            Some(ssl) => Docker::connect_with_ssl(
                &addr,
                &ssl.private_key,
                &ssl.cert_key,
                &ssl.certificate_chain,
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
            remote_host: Some(config),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SslConfig {
    private_key: PathBuf,
    cert_key: PathBuf,
    certificate_chain: PathBuf,
}
