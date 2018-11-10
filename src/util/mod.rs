//! Various utilities to aid in decoding barcodes

#[macro_use]
pub mod macros;

#[allow(unreadable_literal)]
mod chomp;

mod point;

pub mod qr;

pub use self::chomp::Chomp;
pub use self::point::{Delta, Point};
