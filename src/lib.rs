extern crate image;

mod algorithm;
mod decoder;

pub use algorithm::grayscale::{Grayscale, ToLuma};
pub use algorithm::threshold::{BlockedMean, Threshold};

pub use decoder::{default_builder, default_decoder};
pub use decoder::{Decoder, DecoderBuilder};
