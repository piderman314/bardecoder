pub mod galois;

use super::extract::QRData;

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
        String::from("Hello world!")
    }
}
