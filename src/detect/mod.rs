//! Detect various barcodes in pre-processed images
//!
//! (well, only QR so far)

use util::qr::QRLocation;

pub trait Detect<T> {
    fn detect(&self, threshold: &T) -> Vec<Location>;
}

mod linescan;

pub use self::linescan::LineScan;

/// Location of a detected barcode
#[derive(Debug)]
pub enum Location {
    /// Location of a detected QR Code
    QR(QRLocation),
}
