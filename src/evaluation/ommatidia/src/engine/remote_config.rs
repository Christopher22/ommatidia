use super::ReadableFilePath;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct RemoteConfig {
    pub host: String,
    pub port: Option<u16>,
    pub authentication: Option<SslConfig>,
}

impl Default for RemoteConfig {
    fn default() -> Self {
        Self {
            host: "host.docker.internal".into(),
            port: None,
            authentication: None,
        }
    }
}

impl RemoteConfig {
    pub fn port(&self) -> u16 {
        self.port
            .unwrap_or_else(|| match self.authentication.is_none() {
                true => 2375,
                false => 2376,
            })
    }
}

impl std::fmt::Display for RemoteConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", &self.host, self.port())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct SslConfig {
    pub private_key: ReadableFilePath,
    pub cert_key: ReadableFilePath,
    pub certificate_chain: ReadableFilePath,
}
