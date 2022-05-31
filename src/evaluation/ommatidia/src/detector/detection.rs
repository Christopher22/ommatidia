use crate::{dataset::Identifier, detector::Name, Estimate};

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct Detection {
    pub sample: Identifier,
    pub detector: Name,
    #[serde(flatten)]
    pub estimate: Estimate,
}
