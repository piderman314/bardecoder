pub mod galois;

use qr;
use qr::{QRData, QRError};

pub trait Decode {
    fn decode(&self, data: &Vec<QRData>) -> Vec<Result<String, QRError>>;
}

pub struct QRDecoder {}

impl QRDecoder {
    pub fn new() -> QRDecoder {
        QRDecoder {}
    }
}

impl Decode for QRDecoder {
    fn decode(&self, data: &Vec<QRData>) -> Vec<Result<String, QRError>> {
        let mut result = vec![];

        'qr_data: for qr_data in data {
            let format = qr::format::format(&qr_data);

            if format.is_err() {
                result.push(Err(format.err().unwrap()));
                continue;
            }

            let format = format.unwrap();

            let blocks = qr::blocks::blocks(&qr_data, format.1);

            if blocks.is_err() {
                result.push(Err(blocks.err().unwrap()));
                continue;
            }

            let corrected = qr::correct::correct(blocks.unwrap(), qr_data, format.0);

            if corrected.is_err() {
                result.push(Err(corrected.err().unwrap()));
                continue;
            }

            let mut output = String::new();
            for block in corrected.unwrap() {
                let data = qr::data::data(block, qr_data.version);

                if data.is_err() {
                    result.push(Err(data.err().unwrap()));
                    continue 'qr_data;
                }

                output.push_str(data.unwrap().as_str());
            }

            result.push(Ok(output));
        }

        result
    }
}
