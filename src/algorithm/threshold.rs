use image::GrayImage;

pub trait Threshold<G, T> {
    fn to_threshold(&self, grayscale: G) -> T;
}

pub struct BlockedMean {}

impl BlockedMean {
    pub fn new() -> BlockedMean {
        BlockedMean {}
    }
}

impl Threshold<GrayImage, GrayImage> for BlockedMean {
    fn to_threshold(&self, grayscale: GrayImage) -> GrayImage {
        grayscale
    }
}