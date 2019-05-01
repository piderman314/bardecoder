//! Detect various barcodes in pre-processed images
//!
//! (well, only QR so far)

use crate::util::qr::QRLocation;

/// Detect barcode in a prepared image
///
/// PREPD type should be the type if the image returned from the [`Prepare`] implementation
///
/// Pre-implemented Extracts provided by this library that are included in the default [`Detect`]:
/// * [`LineScan`]
///
/// # Example
/// ```
/// # extern crate bardecoder;
/// # extern crate image;
/// # use bardecoder::detect::Location;
/// # use image::GrayImage;
/// use bardecoder::detect::Detect;
///
/// struct MyDetector {}
///
/// impl Detect<GrayImage> for MyDetector {
///     fn detect(&self, prepared: &GrayImage) -> Vec<Location> {
///         // detect codes here
/// #       vec![]
///     }
/// }
/// ```
///
/// with the corresponding impl Detect being the Example [`here`]
///
/// [`Location`]: ../detect/enum.Location.html
/// [`Decode`]: ../decode/trait.Decode.html
/// [`Decoder`]: ../struct.Decoder.html
/// [`Prepare`]: ../prepare/trait.Prepare.html
/// [`here`]: ../detect/trait.Detect.html
pub trait Detect<PREPD> {
    /// Does the actual detecting
    fn detect(&self, prepared: &PREPD) -> Vec<Location>;
}

mod linescan;

pub use self::linescan::LineScan;

/// Location of a detected barcode
#[derive(Debug)]
pub enum Location {
    /// Location of a detected QR Code
    QR(QRLocation),
}
