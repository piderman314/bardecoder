//! Barcode Decoder

#![allow(unknown_lints)]
#![allow(clippy::new_without_default)]
#![allow(clippy::comparison_chain)]
#![warn(missing_docs)]
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

#[macro_use]
extern crate log;

#[macro_use]
extern crate newtype_derive;

mod decoder;

pub mod decode;
pub mod detect;
pub mod extract;
pub mod prepare;
pub mod util;

pub use crate::decoder::{
    default_builder, default_builder_with_info, default_decoder, default_decoder_with_info,
};
pub use crate::decoder::{Decoder, DecoderBuilder};
pub use crate::util::qr::{ECLevel, QRInfo};
