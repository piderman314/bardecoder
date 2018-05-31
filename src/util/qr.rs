use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::ops::Index;

use util::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct QRError {
    pub msg: String,
}

impl Error for QRError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl Display for QRError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "QRError: {}", self.msg)
    }
}

#[derive(Debug)]
pub struct QRData {
    pub data: Vec<u8>,
    pub version: u32,
    pub side: u32,
}

impl QRData {
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

#[derive(Debug)]
pub struct QRLocation {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub module_size: f64,
    pub version: u32,
}
