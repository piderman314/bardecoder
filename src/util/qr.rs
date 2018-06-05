use std::ops::Index;

use util::Point;

/// Generic QR Error message. Can be converted into `failure::Error`
#[derive(Fail, Debug, Clone, PartialEq)]
#[fail(display = "Error decoding QR Code: {}", msg)]
pub struct QRError {
    /// Detail message
    pub msg: String,
}

/// QR Data extracted from the source image
///
/// While the data is still pixels of value 0/255, using the index will convert it into 1's (pixel 0) and 0's (pixel 255)
///
/// # Example
/// ```
/// # extern crate bardecoder;
/// use bardecoder::util::qr::QRData;
///
/// let mut data = vec![0; 21 * 21];
/// data[21 * 21 - 1] = 255;
/// let qr_data = QRData::new(data, 1);
///
/// assert_eq!(qr_data.version, 1);
/// assert_eq!(qr_data.side, 21);
/// assert_eq!(qr_data[[0, 0]], 1);
/// assert_eq!(qr_data[[20, 20]], 0);
/// ```
#[derive(Debug)]
pub struct QRData {
    /// QR Pixel Data in side x side pixels, stored in row major order. Using the provided index will convert into 1's and 0's.
    pub data: Vec<u8>,

    /// Version of the QR Code, 1 being the smallest, 40 the largest
    pub version: u32,

    /// Side in pixels of the QR square
    pub side: u32,
}

impl QRData {
    /// Create a new QRData object with the provided data and version. `side` will be calculated automatically.
    pub fn new(data: Vec<u8>, version: u32) -> QRData {
        QRData {
            data,
            version,
            side: 4 * version + 17,
        }
    }
}

impl Index<[u32; 2]> for QRData {
    type Output = u8;

    fn index(&self, index: [u32; 2]) -> &u8 {
        let pixel = self.data[index[1] as usize * self.side as usize + index[0] as usize];
        if pixel == 0 {
            &1
        } else {
            &0
        }
    }
}

/// Location of the QR Code in the source image, in pixels
#[derive(Debug)]
pub struct QRLocation {
    /// Center of the top left finder pattern, in pixels, relative to the QR Code
    pub top_left: Point,

    /// Center of the top right finder pattern, in pixels, relative to the QR Code
    pub top_right: Point,

    /// Center of the bottom left finder pattern, in pixels, relative to the QR Code
    pub bottom_left: Point,

    /// Module size in pixels
    pub module_size: f64,

    /// Version of the QR Code, 1 being the smallest, 40 the largest
    pub version: u32,
}
