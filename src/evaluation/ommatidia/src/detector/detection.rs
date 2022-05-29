use crate::{dataset::Identifier, detector::Name, Estimate};

#[derive(Debug, PartialEq)]
pub struct Detection {
    pub sample: Identifier,
    pub detector: Name,
    pub estimate: Result<Estimate, String>,
}

impl Detection {
    pub fn estimate(&self) -> Result<&Estimate, &str> {
        self.estimate
            .as_ref()
            .map_err(|error_message| error_message.as_str())
    }
}

impl std::fmt::Display for Detection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.sample.as_ref())
    }
}
