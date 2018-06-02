mod qr;

pub use self::qr::decoder::QRDecoder;

pub trait Decode<DATA, ERROR> {
    fn decode(&self, data: Result<DATA, ERROR>) -> Result<String, ERROR>;
}
