use ommatidia::{
    detector::{Config as DetectorConfig, Detectors, Error as DetectorError},
    engine::Engines,
    serialization::Deserialize,
    Dataset,
};

#[derive(Debug, Deserialize)]
pub struct Config<T = Vec<DetectorConfig>> {
    pub engines: Engines,
    pub detectors: T,
    pub files: Vec<Dataset>,
}

impl Config {
    pub async fn try_spawn(self) -> Result<Config<Detectors>, DetectorError> {
        let detectors = Detectors::try_from(self.detectors, &self.engines).await?;
        Ok(Config {
            engines: self.engines,
            detectors,
            files: self.files,
        })
    }
}
