use util::qr::{QRData, QRError, QRLocation};

mod qr;

pub use self::qr::QRExtractor;

pub trait Extract<T> {
    fn extract(&self, threshold: &T, locs: Vec<QRLocation>) -> Vec<Result<QRData, QRError>>;
}
