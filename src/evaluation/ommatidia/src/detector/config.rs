#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub image: String,
    #[serde(default = "Config::default_config")]
    pub config: serde_json::Value,
    #[serde(default)]
    pub engine: Option<String>,
}

impl Config {
    fn default_config() -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::new())
    }
}

impl AsRef<str> for Config {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialization() {
        let config: super::Config =
            serde_json::from_str(r#" { "name": "test", "image": "test:latest" } "#)
                .expect("valid config");
        assert_eq!(config.name, "test");
        assert_eq!(config.image, "test:latest");
        assert!(config.config.is_object());
        assert!(config.engine.is_none());
    }
}
