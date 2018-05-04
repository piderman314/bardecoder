use image::GrayImage;
use image::DynamicImage;

use super::BlockedMean;
use super::Grayscale;
use super::Threshold;
use super::ToLuma;

pub struct Decoder<S, G, T>
{
    grayscale: Box<Grayscale<S, G>>,
    threshold: Box<Threshold<G, T>>,
}

impl<S, G, T> Decoder<S, G, T>
{
    pub fn new() -> Decoder<DynamicImage, GrayImage, GrayImage> {
        Decoder { grayscale: Box::new(ToLuma::new()) , threshold: Box::new(BlockedMean::new())}
    }

    pub fn decode(&self, source: S) -> G {
        self.grayscale.to_grayscale(source)
    }
}
