//! Barcode Decoder

#![allow(unknown_lints)]
#![allow(new_without_default_derive)]
#![warn(missing_docs)]

extern crate failure;
extern crate image;

#[macro_use]
extern crate log;

#[macro_use]
extern crate failure_derive;

// let util go first since it contains macros
#[macro_use]
pub mod util;

mod decoder;

pub mod decode;
pub mod detect;
pub mod extract;
pub mod prepare;

pub use decoder::{default_builder, default_decoder};
pub use decoder::{Decoder, DecoderBuilder};
