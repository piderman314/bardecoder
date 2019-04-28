//! Prepare an image for data extraction

mod blockedmean;

pub use self::blockedmean::BlockedMean;

/// Prepare the source image for data extraction, for example by converting it to black/white
///
/// IMG type should be the type of the source image
/// PREPD type should be the type of the output image. It does not have to be the same as the IMG type
///
/// Pre-implemented Prepare provided by this library that are included in the default [`Decoder`]:
/// * [`BlockedMean`]
///
/// # Example
/// ```
/// # extern crate bardecoder;
/// # extern crate image;
/// # use image::{DynamicImage, GrayImage};
/// use bardecoder::prepare::Prepare;
///
/// struct MyPreparator {}
///
/// impl Prepare<DynamicImage, GrayImage> for MyPreparator {
///     fn prepare(&self, input: DynamicImage) -> GrayImage {
///         // prepare image here
/// #       input.to_luma()
///     }
/// }
/// ```
///
/// with the corresponding impl Prepare being the Example [`here`]
///
/// [`Decoder`]: ../struct.Decoder.html
/// [`Detect`]: ../detect/trait.Detect.html
/// [`here`]: ../prepare/trait.Prepare.html
pub trait Prepare<IMG, PREPD> {
    /// Does the actual preparing
    fn prepare(&self, source: IMG) -> PREPD;
}
