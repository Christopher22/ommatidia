use crate::{
    detector::{Config, Detector, Error, ErrorType, ShutdownError},
    engine::Engines,
    util::check_duplicates,
};

#[derive(Debug)]
pub struct Detectors(Vec<Detector>);

impl Detectors {
    pub async fn try_from(config: Vec<Config>, engines: &Engines) -> Result<Self, Error> {
        let config =
            check_duplicates(config, |config| &config.name).map_err(|detector_name| Error {
                detector: detector_name,
                details: ErrorType::MultipleNames,
            })?;

        let engine_spawn_jobs = config
            .into_iter()
            .map(|detector_config| {
                let engine = match detector_config.engine {
                    Some(engine) => engines.get(&engine).ok_or_else(|| Error {
                        detector: detector_config.name.clone(),
                        details: ErrorType::UnknownEngine(engine),
                    }),
                    None => engines.get_default().ok_or_else(|| Error {
                        detector: detector_config.name.clone(),
                        details: ErrorType::AmbiguesEngine,
                    }),
                }?;

                Ok(Detector::spawn(
                    detector_config.name,
                    engine,
                    detector_config.image,
                    detector_config.config,
                ))
            })
            .collect::<Result<futures::future::TryJoinAll<_>, _>>()?;

        engine_spawn_jobs.await.map(Detectors)
    }

    pub async fn shutdown(self) -> Vec<ShutdownError> {
        futures::future::join_all(self.0.into_iter().map(|engine| engine.shutdown()))
            .await
            .into_iter()
            .filter_map(|result| result.err())
            .collect()
    }
}

impl AsRef<[Detector]> for Detectors {
    fn as_ref(&self) -> &[Detector] {
        &self.0
    }
}

impl AsMut<[Detector]> for Detectors {
    fn as_mut(&mut self) -> &mut [Detector] {
        &mut self.0
    }
}
