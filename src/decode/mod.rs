mod qr;

pub use self::qr::decoder::QRDecoder;

pub trait Decode<DATA, ERROR> {
    fn decode(&self, data: Vec<Result<DATA, ERROR>>) -> Vec<Result<String, ERROR>>;
}
