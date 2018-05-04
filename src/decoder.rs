use image::GrayImage;
use image::DynamicImage;

use super::Grayscale;
use super::ToLuma;

pub struct Decoder<S>
{
    grayscale: Box<Grayscale<S>>,
}

impl<S> Decoder<S>
{
    pub fn new() -> Decoder<DynamicImage> {
        Decoder { grayscale: Box::new(ToLuma::new()) }
    }

    pub fn decode(&self, source: S) -> GrayImage {
        self.grayscale.to_grayscale(source)
    }
}
