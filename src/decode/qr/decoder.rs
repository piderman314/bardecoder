use super::super::Decode;

use crate::util::qr::{QRData, QRError, QRInfo};

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

impl Decode<QRData, String, QRError> for QRDecoder {
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

/// Decode a QR code into a resulting String. It also includes some information about the decoded QR Code.
///
/// Functions the same as QRDecoder, apart from also returning some information about the decoded QR Code.
pub struct QRDecoderWithInfo {}

impl QRDecoderWithInfo {
    /// Construct a new QRDecoder
    pub fn new() -> QRDecoderWithInfo {
        QRDecoderWithInfo {}
    }
}

impl Decode<QRData, (String, QRInfo), QRError> for QRDecoderWithInfo {
    fn decode(&self, data: Result<QRData, QRError>) -> Result<(String, QRInfo), QRError> {
        let qr_data = data?;

        let format = super::format::format(&qr_data)?;
        let blocks = super::blocks::blocks(&qr_data, &format.0, &format.1)?;
        let block_info = super::block_info(qr_data.version, &format.0)?;

        let mut all_blocks = vec![];
        let mut total_errors = 0;

        for (block, bi) in blocks.into_iter().zip(block_info) {
            let (corrected, error_count) = super::correct::correct_with_error_count(block, &bi)?;

            for corr in corrected.iter().take(bi.data_per as usize) {
                all_blocks.push(*corr);
            }

            total_errors += error_count;
        }

        debug!("TOTAL LENGTH {}", all_blocks.len());
        let total_data = (all_blocks.len() as u32) * 8;

        let data = super::data::data(all_blocks, qr_data.version)?;
        Ok((
            data,
            QRInfo {
                version: qr_data.version,
                ec_level: format.0,
                total_data,
                errors: total_errors,
            },
        ))
    }
}
