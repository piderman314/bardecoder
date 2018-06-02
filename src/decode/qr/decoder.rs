use super::super::Decode;

use util::qr::{QRData, QRError};

pub struct QRDecoder {}

impl QRDecoder {
    pub fn new() -> QRDecoder {
        QRDecoder {}
    }
}

impl Decode<QRData, QRError> for QRDecoder {
    fn decode(&self, data: Vec<Result<QRData, QRError>>) -> Vec<Result<String, QRError>> {
        let mut result = vec![];

        'qr_data: for qr_data in data {
            if qr_data.is_err() {
                result.push(Err(qr_data.err().unwrap()));
                continue;
            }

            let qr_data = qr_data.unwrap();

            let format = super::format::format(&qr_data);

            if format.is_err() {
                result.push(Err(format.err().unwrap()));
                continue;
            }

            let format = format.unwrap();

            let blocks = super::blocks::blocks(&qr_data, &format.0, &format.1);

            if blocks.is_err() {
                result.push(Err(blocks.err().unwrap()));
                continue;
            }

            let mut blocks = blocks.unwrap();

            let block_info = super::block_info(qr_data.version, &format.0);
            if block_info.is_err() {
                result.push(Err(block_info.err().unwrap()));
                continue;
            }

            let block_info = block_info.unwrap();

            let mut all_blocks = vec![];

            let mut b = 0;
            for block in blocks {
                let corrected = super::correct::correct(block, &block_info[b]);

                if corrected.is_err() {
                    result.push(Err(corrected.err().unwrap()));
                    continue; // 'qr_data;
                } else {
                    debug!("BLOCK {} SUCCESFULLY CORRECTED", b);
                }

                let mut corrected = corrected.unwrap();

                for corr in corrected.iter().take(block_info[b].data_per as usize) {
                    all_blocks.push(*corr);
                }

                b += 1;
            }

            debug!("TOTAL LENGTH {}", all_blocks.len());

            let mut output = String::new();
            let data = super::data::data(all_blocks, qr_data.version);

            if data.is_err() {
                result.push(Err(data.err().unwrap()));
                continue 'qr_data;
            }

            output.push_str(data.unwrap().as_str());

            result.push(Ok(output));
        }

        result
    }
}
