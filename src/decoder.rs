use image::DynamicImage;
use image::GrayImage;

use algorithm::decode::Decode;
use algorithm::decode::QRDecoder;
use algorithm::extract::Extract;
use algorithm::extract::QRExtractor;
use detect::Detect;
use detect::LineScan;
use prepare::BlockedMean;
use prepare::Prepare;

use qr::QRError;

pub struct Decoder<IMG, PREPD> {
    prepare: Box<Prepare<IMG, PREPD>>,
    detect: Box<Detect<PREPD>>,
    extract: Box<Extract<PREPD>>,
    decode: Box<Decode>,
}

impl<IMG, PREPD> Decoder<IMG, PREPD> {
    pub fn decode(&self, source: IMG) -> Vec<Result<String, QRError>> {
        let prepared = self.prepare.prepare(source);
        let locations = self.detect.detect(&prepared);

        if locations.is_empty() {
            return vec![];
        }

        let extraction = self.extract.extract(&prepared, locations);
        self.decode.decode(extraction)
    }
}

/// Create a default Decoder
///
/// It will use the following components:
///
/// * prepare: BlockedMean
/// * detect: LineScan
/// * extract: QRExtractor
/// * decode: QRDecoder
///
/// This is meant to provide a good balance between speed and accuracy
pub fn default_decoder() -> Decoder<DynamicImage, GrayImage> {
    default_builder().build()
}

/// Builder struct to create a Decoder
///
/// Required elements are:
///
/// * Prepare
/// * Detect
/// * Extract
/// * Decode
#[derive(Default)]
pub struct DecoderBuilder<IMG, PREPD> {
    prepare: Option<Box<Prepare<IMG, PREPD>>>,
    detect: Option<Box<Detect<PREPD>>>,
    extract: Option<Box<Extract<PREPD>>>,
    decode: Option<Box<Decode>>,
}

#[allow(new_without_default_derive)] // not sure why clippy is complaining about it here
impl<IMG, PREPD> DecoderBuilder<IMG, PREPD> {
    /// Constructor; all fields initialized as None
    pub fn new() -> DecoderBuilder<IMG, PREPD> {
        DecoderBuilder {
            prepare: None,
            detect: None,
            extract: None,
            decode: None,
        }
    }

    pub fn prepare(
        &mut self,
        prepare: Box<Prepare<IMG, PREPD>>,
    ) -> &mut DecoderBuilder<IMG, PREPD> {
        self.prepare = Some(prepare);
        self
    }

    pub fn detect(&mut self, detect: Box<Detect<PREPD>>) -> &mut DecoderBuilder<IMG, PREPD> {
        self.detect = Some(detect);
        self
    }

    pub fn extract(&mut self, extract: Box<Extract<PREPD>>) -> &mut DecoderBuilder<IMG, PREPD> {
        self.extract = Some(extract);
        self
    }

    pub fn decode(&mut self, decode: Box<Decode>) -> &mut DecoderBuilder<IMG, PREPD> {
        self.decode = Some(decode);
        self
    }

    /// Build actual Decoder
    ///
    /// # Panics
    ///
    /// Will panic if any of the required components are missing
    pub fn build(self) -> Decoder<IMG, PREPD> {
        if self.prepare.is_none() {
            panic!("Cannot build Decoder without Prepare component");
        }

        if self.detect.is_none() {
            panic!("Cannot build Decoder without Detect component");
        }

        if self.extract.is_none() {
            panic!("Cannot build Decoder without Extract component");
        }

        if self.decode.is_none() {
            panic!("Cannot build Decoder without Decode componen");
        }

        Decoder {
            prepare: self.prepare.unwrap(),
            detect: self.detect.unwrap(),
            extract: self.extract.unwrap(),
            decode: self.decode.unwrap(),
        }
    }
}

/// Create a default DecoderBuilder
///
/// It will use the following components:
///
/// * prepare: BlockedMean
/// * locate: LineScan
/// * extract: QRExtractor
/// * decode: QRDecoder
///
/// The builder can then be customised before creating the Decoder
pub fn default_builder() -> DecoderBuilder<DynamicImage, GrayImage> {
    let mut db = DecoderBuilder::new();

    db.prepare(Box::new(BlockedMean::new(5, 7)));
    db.detect(Box::new(LineScan::new()));
    db.extract(Box::new(QRExtractor::new()));
    db.decode(Box::new(QRDecoder::new()));

    db
}
