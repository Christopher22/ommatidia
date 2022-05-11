use super::RemoteConfig;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Config {
    pub name: String,
    #[serde(flatten)]
    pub remote_config: Option<RemoteConfig>,
}

#[cfg(test)]
mod tests {
    use super::RemoteConfig;

    #[test]
    fn test_deserialize() {
        let config: super::Config =
            serde_json::from_str(r#" { "name": "test" } "#).expect("valid config");
        assert_eq!(config.name, "test");
        assert_eq!(config.remote_config, None);
    }

    #[test]
    fn test_deserialize_with_remote() {
        let config: super::Config =
            serde_json::from_str(r#" { "name": "test", "host": "fancy_host" } "#)
                .expect("valid config");
        assert_eq!(config.name, "test");
        assert_eq!(
            config.remote_config,
            Some(RemoteConfig {
                host: "fancy_host".into(),
                port: None,
                authentication: None
            })
        );
    }
}
