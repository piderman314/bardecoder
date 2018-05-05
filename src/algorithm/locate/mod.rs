use image::GrayImage;

pub trait Locate<T> {
    fn locate(&self, threshold: &T) -> Vec<QRFinderPosition>;
}

mod linescan;

pub use self::linescan::LineScan;

#[derive(Debug)]
pub enum Location {
    QR(QRLocation),
}

#[derive(Debug)]
pub struct QRLocation {
    top_left: QRFinderPosition,
    top_right: QRFinderPosition,
    bottom_left: QRFinderPosition,
    module_size: u32,
}

#[derive(Debug)]
pub struct QRFinderPosition {
    pub x: u32,
    pub y: u32,
}
