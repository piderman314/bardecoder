extern crate image;

mod algorithm;
mod decoder;

pub use algorithm::grayscale::{Grayscale, ToLuma};
pub use algorithm::threshold::{Threshold, BlockedMean};

pub use decoder::Decoder;
