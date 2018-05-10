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
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub module_size: f64,
    pub version: u32,
}

#[derive(Debug)]
pub struct QRFinderPosition {
    pub location: Point,
    pub module_size: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
