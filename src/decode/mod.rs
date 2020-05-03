//! Decode data extracted from an image

use failure::Fail;

mod qr;

pub use self::qr::decoder::QRDecoder;

/// Decode extracted data into a resulting String
///
/// DATA type must equal the output type of the matching [`Extract`] implementation
///
/// For convenience and easy pipelining the input for decode is the [`Result`] returned from the extract method.
/// Implementors should return any Error passed to the decode function as-is.
///
/// Pre-implemented Decodes provided by this library that are included in the default [`Decoder`]:
/// * [`QRDecoder`]
///
/// # Example
/// ```
/// # extern crate bardecoder;
/// # use bardecoder::util::qr::{QRData, QRError};
/// use bardecoder::decode::Decode;
///
/// struct MyDecoder {}
///
/// impl Decode<QRData, String, QRError> for MyDecoder {
///     fn decode(&self, data: Result<QRData, QRError>) -> Result<String, QRError> {
///         // process data here
/// #        Ok(String::from("ok!"))
///     }
/// }
/// ```
///
/// with the corresponding impl Extract being the Example [`here`]
///
/// [`Extract`]: ../extract/trait.Extract.html
/// [`here`]: ../extract/trait.Extract.html
/// [`Decoder`]: ../struct.Decoder.html

pub trait Decode<DATA, RESULT, ERROR>
where
    ERROR: Fail,
{
    /// Does the actual decoding
    fn decode(&self, data: Result<DATA, ERROR>) -> Result<RESULT, ERROR>;
}
