use qr::QRLocation;

pub trait Detect<T> {
    fn detect(&self, threshold: &T) -> Vec<QRLocation>;
}

mod linescan;

pub use self::linescan::LineScan;

#[derive(Debug)]
pub enum Location {
    QR(QRLocation),
}
