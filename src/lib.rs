extern crate image;

pub mod algorithm;
mod decoder;

pub use decoder::{default_builder, default_decoder};
pub use decoder::{Decoder, DecoderBuilder};
