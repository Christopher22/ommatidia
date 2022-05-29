use std::rc::Rc;

use crate::{detector::Name, Estimate};

#[derive(Debug, PartialEq)]
pub struct Detection {
    identifier: Rc<String>,
    detector: Name,
    estimate: Result<Estimate, String>,
}

impl Detection {
    pub fn ok<S: Into<Rc<String>>>(identifier: S, detector: Name, estimate: Estimate) -> Self {
        Detection {
            identifier: identifier.into(),
            detector,
            estimate: Ok(estimate),
        }
    }

    pub fn failed<S: Into<Rc<String>>>(
        identifier: S,
        detector: Name,
        failure_message: String,
    ) -> Self {
        Detection {
            identifier: identifier.into(),
            detector,
            estimate: Err(failure_message),
        }
    }

    pub fn estimate(&self) -> Result<&Estimate, &str> {
        self.estimate
            .as_ref()
            .map_err(|error_message| error_message.as_str())
    }
}

impl std::fmt::Display for Detection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.identifier.as_str())
    }
}
