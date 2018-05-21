pub mod galois;

use qr;
use qr::QRData;

pub trait Decode {
    fn decode(&self, data: &Vec<QRData>) -> String;
}

pub struct QRDecoder {}

impl QRDecoder {
    pub fn new() -> QRDecoder {
        QRDecoder {}
    }
}

impl Decode for QRDecoder {
    fn decode(&self, data: &Vec<QRData>) -> String {
        let test = qr::format::format(&data[0]).unwrap();
        let codewords = qr::blocks::blocks(&data[0], test.1).unwrap();
        let corrected = qr::correct::correct(codewords, &data[0], test.0).unwrap();
        qr::data::data(corrected[0].clone(), data[0].version).unwrap()
    }
}
