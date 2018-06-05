use util::qr::{QRData, QRError};

pub mod blocks;
pub mod correct;
pub mod data;
pub mod decoder;
pub mod format;
pub mod galois;

#[derive(Debug)]
pub enum ECLevel {
    LOW,
    MEDIUM,
    QUARTILE,
    HIGH,
}

pub type QRMask = Fn(&QRData, u32, u32) -> u8;

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
        // Version 1
        (1, ECLevel::LOW) => Ok(vec![BlockInfo::new(1, 26, 19, 2)]),
        (1, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(1, 26, 16, 4)]),
        (1, ECLevel::QUARTILE) => Ok(vec![BlockInfo::new(1, 26, 13, 6)]),
        (1, ECLevel::HIGH) => Ok(vec![BlockInfo::new(1, 26, 9, 8)]),

        // Version 2
        (2, ECLevel::LOW) => Ok(vec![BlockInfo::new(1, 44, 34, 4)]),
        (2, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(1, 44, 28, 8)]),
        (2, ECLevel::QUARTILE) => Ok(vec![BlockInfo::new(1, 44, 22, 11)]),
        (2, ECLevel::HIGH) => Ok(vec![BlockInfo::new(1, 44, 16, 14)]),

        // Version 3
        (3, ECLevel::LOW) => Ok(vec![BlockInfo::new(1, 70, 55, 7)]),
        (3, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(1, 70, 44, 13)]),
        (3, ECLevel::QUARTILE) => Ok(vec![BlockInfo::new(2, 35, 17, 9)]),
        (3, ECLevel::HIGH) => Ok(vec![BlockInfo::new(2, 35, 13, 11)]),

        // Version 4
        (4, ECLevel::LOW) => Ok(vec![BlockInfo::new(1, 100, 80, 10)]),
        (4, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(2, 50, 32, 9)]),
        (4, ECLevel::QUARTILE) => Ok(vec![BlockInfo::new(2, 50, 24, 13)]),
        (4, ECLevel::HIGH) => Ok(vec![BlockInfo::new(4, 25, 9, 8)]),

        // Version 5
        (5, ECLevel::LOW) => Ok(vec![BlockInfo::new(1, 134, 108, 13)]),
        (5, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(2, 67, 43, 12)]),
        (5, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(2, 33, 15, 9),
            BlockInfo::new(2, 34, 16, 9),
        ]),
        (5, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(2, 33, 11, 11),
            BlockInfo::new(2, 34, 12, 11),
        ]),

        // Version 10
        (10, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(6, 43, 15, 14),
            BlockInfo::new(2, 44, 16, 14),
        ]),

        // Version 25
        (25, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(8, 132, 106, 13),
            BlockInfo::new(4, 133, 107, 13),
        ]),

        // Version 40
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

    for bi in &block_info {
        for _ in 0..bi.block_count {
            bi_unwound.push(bi.clone());
        }
    }

    Ok(bi_unwound)
}
