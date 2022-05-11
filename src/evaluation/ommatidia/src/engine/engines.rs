use super::{Engine, Error as EngineError};

#[derive(Debug)]
pub enum Error {
    DefaultEngineFailed(EngineError),
    DuplicatedName(String),
    DuplicatedHost(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DefaultEngineFailed(error) => write!(
                f,
                "connecting to the local Docker instance (as default given missing explicit specification) failed: {}",
                error
            ),
            Error::DuplicatedName(name) => {
                write!(f, "two Docker hosts share the same name '{}'", name)
            }
            Error::DuplicatedHost(name) => {
                write!(
                    f,
                    "two Docker hosts share the same remote host specification '{}'",
                    name
                )
            }
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(try_from = "Vec<Engine>")]
pub struct Engines(Vec<Engine>);

impl Engines {
    pub const DEFAULT_NAME: &'static str = "default";

    pub fn get<T: AsRef<str>>(&self, name: T) -> Option<Engine> {
        let name = name.as_ref();
        self.0
            .iter()
            .find(|engine| engine.config.name == name)
            .cloned()
    }
}

impl TryFrom<Vec<Engine>> for Engines {
    type Error = Error;

    fn try_from(engines: Vec<Engine>) -> Result<Self, Self::Error> {
        // Try to create the default engine if nothing is defined
        if engines.is_empty() {
            return Engine::local(Engines::DEFAULT_NAME)
                .map(|engine| Engines(vec![engine]))
                .map_err(Error::DefaultEngineFailed);
        }

        // Check for same name
        let engines: Vec<_> =
            crate::util::check_duplicates(engines, |engine| engine.config.name.as_str())
                .map_err(Error::DuplicatedName)?;

        // Check for same (remote) host
        crate::util::check_duplicates(
            engines
                .iter()
                .map(|engine| {
                    engine
                        .config
                        .remote_config
                        .as_ref()
                        .map(|remote| remote.to_string())
                        .unwrap_or_else(|| String::from("local"))
                })
                .collect(),
            |remote| remote.as_str(),
        )
        .map_err(Error::DuplicatedHost)?;

        Ok(Engines(engines))
    }
}
