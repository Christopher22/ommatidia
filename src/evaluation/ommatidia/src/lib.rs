#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod dataset;
pub mod detector;
pub mod engine;
mod estimate;
mod meta_data;
mod util;

pub use self::dataset::{Dataset, Entry, ErrorType as FileError, Sample, Samples};
pub use self::estimate::{Ellipse, Estimate, Estimates, Point, Position, Radian};
pub use self::meta_data::{Error as MetaDataLoadingError, MetaData, OutputType};
pub use self::util::ErrorHandler;

// Re-export for convenient usage
pub use serde as serialization;
