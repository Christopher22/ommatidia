use std::rc::Rc;

use crate::Estimate;

pub struct Detection {
    identifier: Rc<String>,
    estimate: Result<Estimate, String>,
}

impl Detection {
    pub fn ok(identifier: Rc<String>, estimate: Estimate) -> Self {
        Detection {
            identifier,
            estimate: Ok(estimate),
        }
    }

    pub fn failed(identifier: Rc<String>, failure_message: String) -> Self {
        Detection {
            identifier,
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
