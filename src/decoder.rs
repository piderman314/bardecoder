use anyhow::Error;
use image::{GenericImageView, GrayImage, Rgba};

use crate::decode::{Decode, QRDecoder, QRDecoderWithInfo};
use crate::detect::{Detect, LineScan, Location};
use crate::extract::{Extract, QRExtractor};
use crate::prepare::{BlockedMean, Prepare};

use crate::util::qr::{QRData, QRError, QRInfo, QRLocation};

/// Struct to hold logic to do the entire decoding
pub struct Decoder<IMG, PREPD, RESULT> {
    prepare: Box<dyn Prepare<IMG, PREPD>>,
    detect: Box<dyn Detect<PREPD>>,
    qr: ExtractDecode<PREPD, QRLocation, QRData, RESULT, QRError>,
}

impl<IMG, PREPD, RESULT> Decoder<IMG, PREPD, RESULT> {
    /// Do the actual decoding
    ///
    /// Logic is run in the following order:
    /// * prepare
    /// * detect
    /// * per detected code the associated extract and decode functions
    pub fn decode(&self, source: &IMG) -> Vec<Result<RESULT, Error>> {
        let prepared = self.prepare.prepare(source);
        let locations = self.detect.detect(&prepared);

        if locations.is_empty() {
            return vec![];
        }

        let mut all_decoded = vec![];

        for location in locations {
            match location {
                Location::QR(qrloc) => {
                    let extracted = self.qr.extract.extract(&prepared, qrloc);
                    let decoded = self.qr.decode.decode(extracted);

                    all_decoded.push(decoded.map_err(Error::from));
                }
            }
        }

        all_decoded
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
pub fn default_decoder<D>() -> Decoder<D, GrayImage, String> where D: GenericImageView<Pixel = Rgba<u8>> {
    default_builder().build()
}

/// Create a default Decoder that also returns information about the decoded QR Code
///
/// It will use the following components:
///
/// * prepare: BlockedMean
/// * detect: LineScan
/// * extract: QRExtractor
/// * decode: QRDecoderWithInfo
///
/// This is meant to provide a good balance between speed and accuracy
pub fn default_decoder_with_info<D>() -> Decoder<D, GrayImage, (String, QRInfo)> where D: GenericImageView<Pixel = Rgba<u8>> {
    default_builder_with_info().build()
}

/// Builder struct to create a Decoder
///
/// Required elements are:
///
/// * Prepare
/// * Detect
/// * Extract
/// * Decode
pub struct DecoderBuilder<IMG, PREPD, RESULT> {
    prepare: Option<Box<dyn Prepare<IMG, PREPD>>>,
    detect: Option<Box<dyn Detect<PREPD>>>,
    qr: Option<ExtractDecode<PREPD, QRLocation, QRData, RESULT, QRError>>,
}

impl<IMG, PREPD, RESULT> DecoderBuilder<IMG, PREPD, RESULT> {
    /// Constructor; all fields initialized as None
    pub fn new() -> DecoderBuilder<IMG, PREPD, RESULT> {
        DecoderBuilder {
            prepare: None,
            detect: None,
            qr: None,
        }
    }

    /// Set the prepare implementation for this Decoder
    pub fn prepare(
        &mut self,
        prepare: Box<dyn Prepare<IMG, PREPD>>,
    ) -> &mut DecoderBuilder<IMG, PREPD, RESULT> {
        self.prepare = Some(prepare);
        self
    }

    /// Set the detect implementation for this Decoder
    pub fn detect(
        &mut self,
        detect: Box<dyn Detect<PREPD>>,
    ) -> &mut DecoderBuilder<IMG, PREPD, RESULT> {
        self.detect = Some(detect);
        self
    }

    /// Set the extact and decode implementations for this Decoder for QR codes
    pub fn qr(
        &mut self,
        extract: Box<dyn Extract<PREPD, QRLocation, QRData, QRError>>,
        decode: Box<dyn Decode<QRData, RESULT, QRError>>,
    ) -> &mut DecoderBuilder<IMG, PREPD, RESULT> {
        self.qr = Some(ExtractDecode { extract, decode });
        self
    }

    /// Build actual Decoder
    ///
    /// # Panics
    ///
    /// Will panic if any of the required components are missing
    pub fn build(self) -> Decoder<IMG, PREPD, RESULT> {
        if self.prepare.is_none() {
            panic!("Cannot build Decoder without Prepare component");
        }

        if self.detect.is_none() {
            panic!("Cannot build Decoder without Detect component");
        }

        Decoder {
            prepare: self.prepare.unwrap(),
            detect: self.detect.unwrap(),
            qr: self.qr.unwrap(),
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
pub fn default_builder<D>() -> DecoderBuilder<D, GrayImage, String> where D: GenericImageView<Pixel = Rgba<u8>> {
    let mut db = DecoderBuilder::new();

    db.prepare(Box::new(BlockedMean::new(5, 7)));
    db.detect(Box::new(LineScan::new()));
    db.qr(Box::new(QRExtractor::new()), Box::new(QRDecoder::new()));

    db
}

/// Create a default DecoderBuilder that also returns information about the decoded QR Code
///
/// It will use the following components:
///
/// * prepare: BlockedMean
/// * locate: LineScan
/// * extract: QRExtractor
/// * decode: QRDecoderWithInfo
///
/// The builder can then be customised before creating the Decoder
pub fn default_builder_with_info<D>() -> DecoderBuilder<D, GrayImage, (String, QRInfo)> where D: GenericImageView<Pixel = Rgba<u8>> {
    let mut db = DecoderBuilder::new();

    db.prepare(Box::new(BlockedMean::new(5, 7)));
    db.detect(Box::new(LineScan::new()));
    db.qr(
        Box::new(QRExtractor::new()),
        Box::new(QRDecoderWithInfo::new()),
    );

    db
}

struct ExtractDecode<PREPD, LOC, DATA, RESULT, ERROR> {
    extract: Box<dyn Extract<PREPD, LOC, DATA, ERROR>>,
    decode: Box<dyn Decode<DATA, RESULT, ERROR>>,
}
