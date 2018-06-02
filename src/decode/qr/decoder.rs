use super::super::Decode;

use util::qr::{QRData, QRError};

pub struct QRDecoder {}

impl QRDecoder {
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

        let mut b = 0;
        for block in blocks {
            let corrected = super::correct::correct(block, &block_info[b])?;

            for corr in corrected.iter().take(block_info[b].data_per as usize) {
                all_blocks.push(*corr);
            }

            b += 1;
        }

        debug!("TOTAL LENGTH {}", all_blocks.len());

        let data = super::data::data(all_blocks, qr_data.version)?;
        Ok(data)
    }
}
