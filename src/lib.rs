extern crate image;

#[macro_use]
extern crate log;

pub mod algorithm;
pub mod chomp;
mod decoder;
pub mod point;
pub mod qr;

pub use decoder::{default_builder, default_decoder};
pub use decoder::{Decoder, DecoderBuilder};
