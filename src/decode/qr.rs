use crate::util::qr::{QRData, QRError};

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

pub type QRMask = dyn Fn(&QRData, u32, u32) -> u8;

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

        // Version 6
        (6, ECLevel::LOW) => Ok(vec![BlockInfo::new(2, 86, 68, 9)]),
        (6, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(4, 43, 27, 8)]),
        (6, ECLevel::QUARTILE) => Ok(vec![BlockInfo::new(4, 43, 19, 12)]),
        (6, ECLevel::HIGH) => Ok(vec![BlockInfo::new(4, 43, 15, 14)]),

        // Version 7
        (7, ECLevel::LOW) => Ok(vec![BlockInfo::new(2, 98, 78, 10)]),
        (7, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(4, 49, 31, 9)]),
        (7, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(2, 32, 14, 9),
            BlockInfo::new(4, 33, 15, 9),
        ]),
        (7, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(4, 39, 13, 13),
            BlockInfo::new(1, 40, 14, 13),
        ]),

        // Version 8
        (8, ECLevel::LOW) => Ok(vec![BlockInfo::new(2, 121, 97, 12)]),
        (8, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(2, 60, 38, 11),
            BlockInfo::new(2, 61, 39, 11),
        ]),
        (8, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(4, 40, 18, 11),
            BlockInfo::new(2, 41, 19, 11),
        ]),
        (8, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(4, 40, 14, 13),
            BlockInfo::new(2, 41, 15, 13),
        ]),

        // Version 9
        (9, ECLevel::LOW) => Ok(vec![BlockInfo::new(2, 146, 116, 15)]),
        (9, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(3, 58, 36, 11),
            BlockInfo::new(2, 59, 37, 11),
        ]),
        (9, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(4, 36, 16, 10),
            BlockInfo::new(4, 37, 17, 10),
        ]),
        (9, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(4, 36, 12, 12),
            BlockInfo::new(4, 37, 13, 12),
        ]),

        // Version 10
        (10, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(2, 86, 68, 9),
            BlockInfo::new(2, 87, 69, 9),
        ]),
        (10, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(4, 69, 43, 13),
            BlockInfo::new(1, 70, 44, 13),
        ]),
        (10, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(6, 43, 19, 12),
            BlockInfo::new(2, 44, 20, 12),
        ]),
        (10, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(6, 43, 15, 14),
            BlockInfo::new(2, 44, 16, 14),
        ]),

        // Version 11
        (11, ECLevel::LOW) => Ok(vec![BlockInfo::new(4, 101, 81, 10)]),
        (11, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(1, 80, 50, 15),
            BlockInfo::new(4, 81, 51, 15),
        ]),
        (11, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(4, 50, 22, 14),
            BlockInfo::new(4, 51, 23, 14),
        ]),
        (11, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(3, 36, 12, 12),
            BlockInfo::new(8, 37, 13, 12),
        ]),

        // Version 12
        (12, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(2, 116, 92, 12),
            BlockInfo::new(2, 117, 93, 12),
        ]),
        (12, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(6, 58, 36, 11),
            BlockInfo::new(2, 59, 37, 11),
        ]),
        (12, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(4, 46, 20, 13),
            BlockInfo::new(6, 47, 21, 13),
        ]),
        (12, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(7, 42, 14, 14),
            BlockInfo::new(4, 43, 15, 14),
        ]),

        // Version 13
        (13, ECLevel::LOW) => Ok(vec![BlockInfo::new(4, 133, 107, 13)]),
        (13, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(8, 59, 37, 11),
            BlockInfo::new(1, 60, 38, 11),
        ]),
        (13, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(8, 44, 20, 12),
            BlockInfo::new(4, 45, 21, 12),
        ]),
        (13, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(12, 33, 11, 11),
            BlockInfo::new(4, 34, 12, 11),
        ]),

        // Version 14
        (14, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(3, 145, 115, 15),
            BlockInfo::new(1, 146, 116, 15),
        ]),
        (14, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(4, 64, 40, 12),
            BlockInfo::new(5, 65, 41, 12),
        ]),
        (14, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(11, 36, 16, 10),
            BlockInfo::new(5, 37, 17, 10),
        ]),
        (14, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(11, 36, 12, 12),
            BlockInfo::new(5, 37, 13, 12),
        ]),

        // Version 15
        (15, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(5, 109, 87, 11),
            BlockInfo::new(1, 110, 88, 11),
        ]),
        (15, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(5, 65, 41, 12),
            BlockInfo::new(5, 66, 42, 12),
        ]),
        (15, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(5, 54, 24, 15),
            BlockInfo::new(7, 55, 25, 15),
        ]),
        (15, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(11, 36, 12, 12),
            BlockInfo::new(7, 37, 13, 12),
        ]),

        // Version 16
        (16, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(5, 122, 98, 12),
            BlockInfo::new(1, 123, 99, 12),
        ]),
        (16, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(7, 73, 45, 14),
            BlockInfo::new(3, 74, 46, 14),
        ]),
        (16, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(15, 43, 19, 12),
            BlockInfo::new(2, 44, 20, 12),
        ]),
        (16, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(3, 45, 15, 15),
            BlockInfo::new(13, 46, 16, 15),
        ]),

        // Version 17
        (17, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(1, 135, 107, 14),
            BlockInfo::new(5, 136, 108, 14),
        ]),
        (17, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(10, 74, 46, 14),
            BlockInfo::new(1, 75, 47, 14),
        ]),
        (17, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(1, 50, 22, 14),
            BlockInfo::new(15, 51, 23, 14),
        ]),
        (17, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(2, 42, 14, 14),
            BlockInfo::new(17, 43, 15, 14),
        ]),

        // Version 18
        (18, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(5, 150, 120, 15),
            BlockInfo::new(1, 151, 121, 15),
        ]),
        (18, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(9, 69, 43, 13),
            BlockInfo::new(4, 70, 44, 13),
        ]),
        (18, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(17, 50, 22, 14),
            BlockInfo::new(1, 51, 23, 14),
        ]),
        (18, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(2, 42, 14, 14),
            BlockInfo::new(19, 43, 15, 14),
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
