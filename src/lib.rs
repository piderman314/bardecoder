#![allow(unknown_lints)]
#![allow(new_without_default_derive)]
#![warn(missing_docs)]

extern crate image;

#[macro_use]
extern crate log;

mod decoder;

pub mod decode;
pub mod detect;
pub mod extract;
pub mod prepare;
pub mod util;

pub use decoder::{default_builder, default_decoder};
pub use decoder::{Decoder, DecoderBuilder};
