#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod connection;
pub mod detector;
pub mod engine;
mod errors;
mod estimate;
mod files;
mod meta_data;
mod util;

pub use self::detector::Detector;

pub use self::errors::{ErrorWithId, Errors};
pub use self::estimate::{Ellipse, Estimate, Point, Position, Radian};
pub use self::files::{Entry, Error as FileError, Files, Sample, Samples};
pub use self::meta_data::{Error as MetaDataLoadingError, MetaData, OutputType, Training};
