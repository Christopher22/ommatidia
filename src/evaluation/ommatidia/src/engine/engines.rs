use super::{Engine, Error};

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

    pub fn get_default(&self) -> Option<Engine> {
        if self.0.len() == 1 {
            return Some(self.0[0].clone());
        }
        self.get(Engines::DEFAULT_NAME)
    }
}

impl TryFrom<Vec<Engine>> for Engines {
    type Error = Error;

    fn try_from(engines: Vec<Engine>) -> Result<Self, Self::Error> {
        // Try to create the default engine if nothing is defined
        if engines.is_empty() {
            return Engine::local(Engines::DEFAULT_NAME)
                .map(|engine| Engines(vec![engine]))
                .map_err(Error::from);
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
