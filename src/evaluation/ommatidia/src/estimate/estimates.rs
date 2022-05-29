use crate::{
    detector::{Detection, DetectionError, Detectors},
    Dataset, ErrorHandler,
};

/// Collect the detections (= failable `Estimate`s) from different detectors.
#[derive(Debug)]
pub struct Estimates<'a, E: ErrorHandler> {
    estimates: Vec<Result<Vec<Detection>, DetectionError>>,
    current_estimator: Vec<Detection>,
    error_handler: &'a E,
}

impl<'a, E: ErrorHandler> Estimates<'a, E> {
    fn new(
        mut estimates: Vec<Result<Vec<Detection>, DetectionError>>,
        error_handler: &'a E,
    ) -> Self {
        while let Some(next_estimate) = estimates.pop() {
            match next_estimate {
                Ok(current_estimate) => {
                    return Self {
                        estimates,
                        current_estimator: current_estimate,
                        error_handler,
                    }
                }
                Err(error) => error_handler.handle(error),
            }
        }

        Self {
            estimates,
            current_estimator: Vec::new(),
            error_handler,
        }
    }

    pub async fn load(
        detectors: &mut Detectors,
        mut input: Dataset,
        errors: &'a E,
    ) -> Estimates<'a, E> {
        let (_, estimates) = {
            let detections: futures::future::JoinAll<_> = detectors
                .as_mut()
                .iter_mut()
                .map(|detector| detector.detect(input.create_connection()))
                .collect();
            let loader = input.load(errors);
            tokio::join!(loader, detections)
        };

        Estimates::new(estimates, errors)
    }
}

impl<'a, E: ErrorHandler> Iterator for Estimates<'a, E> {
    type Item = Detection;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next_estimate) = self.current_estimator.pop() {
                return Some(next_estimate);
            }

            match self.estimates.pop() {
                Some(Ok(next_estimate)) => {
                    self.current_estimator = next_estimate;
                }
                Some(Err(error)) => {
                    self.error_handler.handle(error);
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_elements = self
            .estimates
            .iter()
            .map(|detections| {
                detections
                    .as_ref()
                    .map(|detections| detections.len())
                    .unwrap_or(0usize)
            })
            .sum::<usize>()
            + self.current_estimator.len();

        (remaining_elements, Some(remaining_elements))
    }
}

impl<'a, E: ErrorHandler> ExactSizeIterator for Estimates<'a, E> {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::Estimates;
    use crate::{
        detector::{Detection, Name},
        ErrorHandler, Point,
    };

    #[derive(Default)]
    struct ErrorCounter(pub RefCell<u32>);
    impl ErrorHandler for ErrorCounter {
        fn handle<E: std::error::Error>(&self, _: E) {
            *self.0.borrow_mut() += 1;
        }
    }

    struct IgnoreError;
    impl ErrorHandler for IgnoreError {
        fn handle<E: std::error::Error>(&self, _: E) {}
    }
    // A workaround to allow static lifetimes
    const IGNORE_ERROR: &IgnoreError = &IgnoreError;

    fn generate_data() -> Estimates<'static, IgnoreError> {
        let detector_name = Name::try_from("detector").expect("valid name");
        let example_data = vec![Ok(vec![
            Detection::ok(
                String::from("identifier1"),
                detector_name.clone(),
                crate::Estimate::Point(Point {
                    pos: crate::Position { x: 1, y: 2 },
                    confidence: Some(1.0),
                }),
            ),
            Detection::failed(
                String::from("identifier2"),
                detector_name.clone(),
                String::from("An error"),
            ),
            Detection::ok(
                String::from("identifier3"),
                detector_name,
                crate::Estimate::Point(Point {
                    pos: crate::Position { x: 1, y: 2 },
                    confidence: Some(0.7),
                }),
            ),
        ])];

        Estimates::new(example_data, IGNORE_ERROR)
    }

    #[test]
    fn test_size_hint() {
        let estimates = generate_data();
        assert_eq!(estimates.size_hint(), (3, Some(3)));
    }

    #[test]
    fn test_values() {
        let mut estimates = generate_data();
        assert!(estimates.any(|detection| detection.identifier.as_str() == "identifier3"));
    }

    #[test]
    fn test_number_of_values() {
        let estimates = generate_data();
        assert_eq!(estimates.count(), 3);
    }
}
