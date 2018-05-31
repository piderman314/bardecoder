use util::qr::{QRData, QRError};

mod qr;

pub use self::qr::decoder::QRDecoder;

pub trait Decode {
    fn decode(&self, data: Vec<Result<QRData, QRError>>) -> Vec<Result<String, QRError>>;
}
