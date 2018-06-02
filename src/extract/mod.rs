mod qr;

pub use self::qr::QRExtractor;

pub trait Extract<T, LOC, DATA, ERROR> {
    fn extract(&self, threshold: &T, locs: Vec<LOC>) -> Vec<Result<DATA, ERROR>>;
}
