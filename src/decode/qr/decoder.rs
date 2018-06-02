use super::super::Decode;

use util::qr::{QRData, QRError};

/// Decode a QR code into a resulting String
///
/// This decoder will, in order:
/// * Determine QR Format information
/// * Extract the interleaved blocks of codewords
/// * Perform error correction
/// * Decode the blocks into a String
///
/// # Optimisation
/// The error correction process can be relatively expensive. This decoder has a fast detection of the existence of errors,
/// allowing to bypass the correction altogether if none exist. Users of this library are encouraged to provide high quality fault-free images,
/// speeding up the decoding process by not having to correct errors.
pub struct QRDecoder {}

impl QRDecoder {
    /// Construct a new QRDecoder
    pub fn new() -> QRDecoder {
        QRDecoder {}
    }
}

impl Decode<QRData, QRError> for QRDecoder {
    fn decode(&self, data: Result<QRData, QRError>) -> Result<String, QRError> {
        let qr_data = data?;

        let format = super::format::format(&qr_data)?;
        let blocks = super::blocks::blocks(&qr_data, &format.0, &format.1)?;
        let block_info = super::block_info(qr_data.version, &format.0)?;

        let mut all_blocks = vec![];

        for (block, bi) in blocks.into_iter().zip(block_info) {
            let corrected = super::correct::correct(block, &bi)?;

            for corr in corrected.iter().take(bi.data_per as usize) {
                all_blocks.push(*corr);
            }
        }

        debug!("TOTAL LENGTH {}", all_blocks.len());

        let data = super::data::data(all_blocks, qr_data.version)?;
        Ok(data)
    }
}
