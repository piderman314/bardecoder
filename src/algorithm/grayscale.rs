use image::DynamicImage;
use image::GrayImage;

pub trait Grayscale<S, G> {
    fn to_grayscale(&self, source: S) -> G;
}

pub struct ToLuma {}

impl ToLuma {
    pub fn new() -> ToLuma {
        ToLuma {}
    }
}

impl Grayscale<DynamicImage, GrayImage> for ToLuma {
    fn to_grayscale(&self, source: DynamicImage) -> GrayImage {
        source.to_luma()
    }
}
