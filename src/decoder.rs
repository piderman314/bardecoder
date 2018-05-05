use image::DynamicImage;
use image::GrayImage;

use super::BlockedMean;
use super::Grayscale;
use super::Threshold;
use super::ToLuma;

pub struct Decoder<S, G, T> {
    grayscale: Box<Grayscale<S, G>>,
    threshold: Box<Threshold<G, T>>,
}

impl<S, G, T> Decoder<S, G, T> {
    pub fn decode(&self, source: S) -> T {
        let grayscale = self.grayscale.to_grayscale(source);
        self.threshold.to_threshold(grayscale)
    }
}

/// Create a default Decoder
///
/// It will use the following components:
///
/// * grayscale: ToLuma
/// * threshold: BlockedMean
///
/// This is meant to provide a good balance between speed and accuracy
pub fn default_decoder() -> Decoder<DynamicImage, GrayImage, GrayImage> {
    default_builder().build()
}

/// Builder struct to create a Decoder
///
/// Required elements are:
///
/// * Grayscale
/// * Threshold
pub struct DecoderBuilder<S, G, T> {
    grayscale: Option<Box<Grayscale<S, G>>>,
    threshold: Option<Box<Threshold<G, T>>>,
}

impl<S, G, T> DecoderBuilder<S, G, T> {
    /// Constructor; all fields initialized as None
    pub fn new() -> DecoderBuilder<S, G, T> {
        DecoderBuilder {
            grayscale: None,
            threshold: None,
        }
    }

    /// Add Grayscale component
    pub fn grayscale(&mut self, grayscale: Box<Grayscale<S, G>>) -> &mut DecoderBuilder<S, G, T> {
        self.grayscale = Some(grayscale);
        self
    }

    /// Add Threshold component
    pub fn threshold(&mut self, threshold: Box<Threshold<G, T>>) -> &mut DecoderBuilder<S, G, T> {
        self.threshold = Some(threshold);
        self
    }

    /// Build actual Decoder
    ///
    /// # Panics
    ///
    /// Will panic if any of the required components are missing
    pub fn build(self) -> Decoder<S, G, T> {
        if self.grayscale.is_none() {
            panic!("Cannot build Decoder without Grayscale component");
        }

        if self.threshold.is_none() {
            panic!("Cannot build Decoder without Threshold component");
        }

        Decoder {
            grayscale: self.grayscale.unwrap(),
            threshold: self.threshold.unwrap(),
        }
    }
}

/// Create a default DecoderBuilder
///
/// It will use the following components:
///
/// * grayscale: ToLuma
/// * threshold: BlockedMean
///
/// The builder can then be customised before creating the Decoder
pub fn default_builder() -> DecoderBuilder<DynamicImage, GrayImage, GrayImage> {
    let mut db = DecoderBuilder::new();

    db.grayscale(Box::new(ToLuma::new()));
    db.threshold(Box::new(BlockedMean::new(5, 7)));

    db
}
