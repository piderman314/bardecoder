use std::error::Error;
use std::fmt;
use std::ops::Index;

use point::Point;

use self::format::ECLevel;

pub mod blocks;
pub mod correct;
pub mod data;
pub mod format;

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

#[derive(Debug)]
pub struct QRFinderPosition {
    pub location: Point,
    pub module_size: f64,
    pub last_module_size: f64,
}

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub block_count: u8,
    pub total_per: u8,
    pub data_per: u8,
    pub ec_cap: u8,
}

impl BlockInfo {
    pub fn new(block_count: u8, total_per: u8, data_per: u8, ec_cap: u8) -> BlockInfo {
        BlockInfo {
            block_count,
            total_per,
            data_per,
            ec_cap,
        }
    }
}

pub fn block_info(version: u32, level: &ECLevel) -> Result<Vec<BlockInfo>, QRError> {
    let block_info = match (version, level) {
        (1, ECLevel::LOW) => Ok(vec![BlockInfo::new(1, 26, 19, 2)]),
        (1, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(1, 26, 16, 4)]),
        (1, ECLevel::HIGH) => Ok(vec![BlockInfo::new(1, 26, 9, 8)]),
        (2, ECLevel::HIGH) => Ok(vec![BlockInfo::new(1, 44, 16, 14)]),
        (3, ECLevel::LOW) => Ok(vec![BlockInfo::new(1, 70, 55, 7)]),
        (3, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(1, 70, 44, 13)]),
        (3, ECLevel::QUARTILE) => Ok(vec![BlockInfo::new(2, 35, 17, 9)]),
        (3, ECLevel::HIGH) => Ok(vec![BlockInfo::new(2, 35, 13, 11)]),
        (4, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(2, 50, 32, 9)]),
        (4, ECLevel::HIGH) => Ok(vec![BlockInfo::new(4, 25, 9, 8)]),
        (10, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(6, 43, 15, 14),
            BlockInfo::new(2, 44, 16, 14),
        ]),
        (25, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(8, 132, 106, 13),
            BlockInfo::new(4, 133, 107, 13),
        ]),
        (40, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(20, 45, 15, 15),
            BlockInfo::new(61, 46, 16, 15),
        ]),
        (version, level) => Err(QRError {
            msg: format!(
                "Unknown combination of version {} and level {:?}",
                version, level
            ),
        }),
    }?;

    let mut bi_unwound = vec![];

    for bi in block_info.iter() {
        for _ in 0..bi.block_count {
            bi_unwound.push(bi.clone());
        }
    }

    Ok(bi_unwound)
}

#[derive(Debug, Clone, PartialEq)]
pub struct QRError {
    msg: String,
}

impl Error for QRError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for QRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QRError: {}", self.msg)
    }
}
