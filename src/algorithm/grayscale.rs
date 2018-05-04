use image::DynamicImage;
use image::GrayImage;

pub trait Grayscale<S> {
    fn to_grayscale(&self, source: S) -> GrayImage;
}

pub struct ToLuma {}

impl ToLuma {
    pub fn new() -> ToLuma {
        ToLuma {}
    }
}

impl Grayscale<DynamicImage> for ToLuma {
    fn to_grayscale(&self, source: DynamicImage) -> GrayImage {
        source.to_luma()
    }
}