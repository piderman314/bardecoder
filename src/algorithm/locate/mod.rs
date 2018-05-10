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

#[derive(Debug)]
pub struct QRLocation {
    pub top_left: (u32, u32),
    pub top_right: (u32, u32),
    pub bottom_left: (u32, u32),
    pub module_size: f64,
    pub version: u32,
}

#[derive(Debug)]
pub struct QRFinderPosition {
    pub x: u32,
    pub y: u32,
    module_size: f64,
}
