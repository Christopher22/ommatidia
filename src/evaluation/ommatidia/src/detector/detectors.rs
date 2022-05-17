use crate::{
    detector::{Config, CreationError, Detector},
    engine::Engines,
    util::check_duplicates,
};

#[derive(Debug, Clone)]
pub enum Error {
    MultipleNames(String),
    UnknownEngine(String, String),
    AmbiguesEngine(String),
    DetectorError(CreationError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MultipleNames(name) => {
                write!(f, "detector with name '{}' exists multiple times", name)
            }
            Error::UnknownEngine(name, engine) => write!(
                f,
                "an the engine '{}' for detector '{}' is not known",
                engine, name
            ),
            Error::AmbiguesEngine(name) => write!(f, "unable to select a default engine for detector '{}' without explicit specification", name),
            Error::DetectorError(error) => write!(f, "spawing the detector failed: {}", error)
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Detectors(Vec<Detector>);

impl Detectors {
    pub async fn try_from(config: Vec<Config>, engines: &Engines) -> Result<Self, Error> {
        let config = check_duplicates(config, |config| config.name.as_str())
            .map_err(Error::MultipleNames)?;

        let engine_spawn_jobs = config
            .into_iter()
            .map(|detector_config| {
                let engine = match detector_config.engine {
                    Some(engine) => engines
                        .get(&engine)
                        .ok_or_else(|| Error::UnknownEngine(detector_config.name.clone(), engine)),
                    None => engines
                        .get_default()
                        .ok_or_else(|| Error::AmbiguesEngine(detector_config.name.clone())),
                }?;

                Ok(Detector::spawn(
                    detector_config.name,
                    engine,
                    detector_config.image,
                    detector_config.config,
                ))
            })
            .collect::<Result<futures::future::TryJoinAll<_>, _>>()?;

        engine_spawn_jobs
            .await
            .map(Detectors)
            .map_err(Error::DetectorError)
    }
}

impl AsRef<[Detector]> for Detectors {
    fn as_ref(&self) -> &[Detector] {
        &self.0
    }
}
