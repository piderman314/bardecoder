use point::Point;
use qr::QRLocation;

use image::GrayImage;

pub trait Locate<T> {
    fn locate(&self, threshold: &T) -> Vec<QRLocation>;
}

mod linescan;

pub use self::linescan::LineScan;

#[derive(Debug)]
pub enum Location {
    QR(QRLocation),
}
