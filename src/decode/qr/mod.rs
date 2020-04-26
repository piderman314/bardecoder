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

        // Version 19
        (19, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(3, 141, 113, 14),
            BlockInfo::new(4, 142, 114, 14),
        ]),
        (19, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(3, 70, 44, 13),
            BlockInfo::new(11, 71, 45, 13),
        ]),
        (19, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(17, 47, 21, 13),
            BlockInfo::new(4, 48, 22, 13),
        ]),
        (19, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(9, 39, 13, 13),
            BlockInfo::new(16, 40, 14, 13),
        ]),

        // Version 20
        (20, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(3, 135, 107, 14),
            BlockInfo::new(5, 136, 108, 14),
        ]),
        (20, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(3, 67, 41, 13),
            BlockInfo::new(13, 68, 42, 13),
        ]),
        (20, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(15, 54, 24, 15),
            BlockInfo::new(5, 55, 25, 15),
        ]),
        (20, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(15, 43, 15, 14),
            BlockInfo::new(10, 44, 16, 14),
        ]),

        // Version 21
        (21, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(4, 144, 116, 14),
            BlockInfo::new(4, 145, 117, 14),
        ]),
        (21, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(17, 68, 42, 13)]),
        (21, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(17, 50, 22, 14),
            BlockInfo::new(6, 51, 23, 14),
        ]),
        (21, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(19, 46, 16, 15),
            BlockInfo::new(6, 47, 17, 15),
        ]),

        // Version 22
        (22, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(2, 139, 111, 14),
            BlockInfo::new(7, 140, 112, 14),
        ]),
        (22, ECLevel::MEDIUM) => Ok(vec![BlockInfo::new(17, 74, 46, 14)]),
        (22, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(7, 54, 24, 15),
            BlockInfo::new(16, 55, 25, 15),
        ]),
        (22, ECLevel::HIGH) => Ok(vec![BlockInfo::new(34, 37, 13, 12)]),

        // Version 23
        (23, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(4, 151, 121, 15),
            BlockInfo::new(5, 152, 122, 15),
        ]),
        (23, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(4, 75, 47, 14),
            BlockInfo::new(14, 76, 48, 14),
        ]),
        (23, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(11, 54, 24, 15),
            BlockInfo::new(14, 55, 25, 15),
        ]),
        (23, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(16, 45, 15, 15),
            BlockInfo::new(14, 46, 16, 15),
        ]),

        // Version 24
        (24, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(6, 147, 117, 15),
            BlockInfo::new(4, 148, 118, 15),
        ]),
        (24, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(6, 73, 45, 14),
            BlockInfo::new(14, 74, 46, 14),
        ]),
        (24, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(11, 54, 24, 15),
            BlockInfo::new(16, 55, 25, 15),
        ]),
        (24, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(30, 46, 16, 15),
            BlockInfo::new(2, 47, 17, 15),
        ]),

        // Version 25
        (25, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(8, 132, 106, 13),
            BlockInfo::new(4, 133, 107, 13),
        ]),
        (25, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(8, 75, 47, 14),
            BlockInfo::new(13, 76, 48, 14),
        ]),
        (25, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(7, 54, 24, 15),
            BlockInfo::new(22, 55, 25, 15),
        ]),
        (25, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(22, 45, 15, 15),
            BlockInfo::new(13, 46, 16, 15),
        ]),

        // Version 26
        (26, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(10, 142, 114, 14),
            BlockInfo::new(2, 143, 115, 14),
        ]),
        (26, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(19, 74, 46, 14),
            BlockInfo::new(4, 75, 47, 14),
        ]),
        (26, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(28, 50, 22, 14),
            BlockInfo::new(6, 51, 23, 14),
        ]),
        (26, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(33, 46, 16, 15),
            BlockInfo::new(4, 47, 17, 15),
        ]),

        // Version 27
        (27, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(8, 152, 122, 15),
            BlockInfo::new(4, 153, 123, 15),
        ]),
        (27, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(22, 73, 45, 14),
            BlockInfo::new(3, 74, 46, 14),
        ]),
        (27, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(8, 53, 23, 15),
            BlockInfo::new(26, 54, 24, 15),
        ]),
        (27, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(12, 45, 15, 15),
            BlockInfo::new(28, 46, 16, 15),
        ]),

        // Version 28
        (28, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(3, 147, 117, 15),
            BlockInfo::new(10, 148, 118, 15),
        ]),
        (28, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(3, 73, 45, 14),
            BlockInfo::new(23, 74, 46, 14),
        ]),
        (28, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(4, 54, 24, 15),
            BlockInfo::new(31, 55, 25, 15),
        ]),
        (28, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(11, 45, 15, 15),
            BlockInfo::new(31, 46, 16, 15),
        ]),

        // Version 29
        (29, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(7, 146, 116, 15),
            BlockInfo::new(7, 147, 117, 15),
        ]),
        (29, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(21, 73, 45, 14),
            BlockInfo::new(7, 74, 46, 14),
        ]),
        (29, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(1, 53, 23, 15),
            BlockInfo::new(37, 54, 24, 15),
        ]),
        (29, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(19, 45, 15, 15),
            BlockInfo::new(26, 46, 16, 15),
        ]),

        // Version 30
        (30, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(5, 145, 115, 15),
            BlockInfo::new(10, 146, 116, 15),
        ]),
        (30, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(19, 75, 47, 14),
            BlockInfo::new(10, 76, 48, 14),
        ]),
        (30, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(15, 54, 24, 15),
            BlockInfo::new(25, 55, 25, 15),
        ]),
        (30, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(23, 45, 15, 15),
            BlockInfo::new(25, 46, 16, 15),
        ]),

        // Version 31
        (31, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(13, 145, 115, 15),
            BlockInfo::new(3, 146, 116, 15),
        ]),
        (31, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(2, 74, 46, 14),
            BlockInfo::new(29, 75, 47, 14),
        ]),
        (31, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(42, 54, 24, 15),
            BlockInfo::new(1, 55, 25, 15),
        ]),
        (31, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(23, 45, 15, 15),
            BlockInfo::new(28, 46, 16, 15),
        ]),

        // Version 32
        (32, ECLevel::LOW) => Ok(vec![BlockInfo::new(17, 145, 115, 15)]),
        (32, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(10, 74, 46, 14),
            BlockInfo::new(23, 75, 47, 14),
        ]),
        (32, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(10, 54, 24, 15),
            BlockInfo::new(35, 55, 25, 15),
        ]),
        (32, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(19, 45, 15, 15),
            BlockInfo::new(35, 46, 16, 15),
        ]),

        // Version 33
        (33, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(17, 145, 115, 15),
            BlockInfo::new(1, 146, 116, 15),
        ]),
        (33, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(14, 74, 46, 14),
            BlockInfo::new(21, 75, 47, 14),
        ]),
        (33, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(29, 54, 24, 15),
            BlockInfo::new(19, 55, 25, 15),
        ]),
        (33, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(11, 45, 15, 15),
            BlockInfo::new(46, 46, 16, 15),
        ]),

        // Version 34
        (34, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(13, 145, 115, 15),
            BlockInfo::new(6, 146, 116, 15),
        ]),
        (34, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(14, 74, 46, 14),
            BlockInfo::new(23, 75, 47, 14),
        ]),
        (34, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(44, 54, 24, 15),
            BlockInfo::new(7, 55, 25, 15),
        ]),
        (34, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(59, 46, 16, 15),
            BlockInfo::new(1, 47, 17, 15),
        ]),

        // Version 35
        (35, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(12, 151, 121, 15),
            BlockInfo::new(7, 152, 122, 15),
        ]),
        (35, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(12, 75, 47, 14),
            BlockInfo::new(26, 76, 48, 14),
        ]),
        (35, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(39, 54, 24, 15),
            BlockInfo::new(14, 55, 25, 15),
        ]),
        (35, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(22, 45, 15, 15),
            BlockInfo::new(41, 46, 16, 15),
        ]),

        // Version 36
        (36, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(6, 151, 121, 15),
            BlockInfo::new(14, 152, 122, 15),
        ]),
        (36, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(6, 75, 47, 14),
            BlockInfo::new(34, 76, 48, 14),
        ]),
        (36, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(46, 54, 24, 15),
            BlockInfo::new(10, 55, 25, 15),
        ]),
        (36, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(2, 45, 15, 15),
            BlockInfo::new(64, 46, 16, 15),
        ]),

        // Version 37
        (37, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(17, 152, 122, 15),
            BlockInfo::new(4, 153, 123, 15),
        ]),
        (37, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(29, 74, 46, 14),
            BlockInfo::new(14, 75, 47, 14),
        ]),
        (37, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(49, 54, 24, 15),
            BlockInfo::new(10, 55, 25, 15),
        ]),
        (37, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(24, 45, 15, 15),
            BlockInfo::new(46, 46, 16, 15),
        ]),

        // Version 38
        (38, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(4, 152, 122, 15),
            BlockInfo::new(18, 153, 123, 15),
        ]),
        (38, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(13, 74, 46, 14),
            BlockInfo::new(32, 75, 47, 14),
        ]),
        (38, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(48, 54, 24, 15),
            BlockInfo::new(14, 55, 25, 15),
        ]),
        (38, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(42, 45, 15, 15),
            BlockInfo::new(32, 46, 16, 15),
        ]),

        // Version 39
        (39, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(20, 147, 117, 15),
            BlockInfo::new(4, 148, 118, 15),
        ]),
        (39, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(40, 75, 47, 14),
            BlockInfo::new(7, 76, 48, 14),
        ]),
        (39, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(43, 54, 24, 15),
            BlockInfo::new(22, 55, 25, 15),
        ]),
        (39, ECLevel::HIGH) => Ok(vec![
            BlockInfo::new(10, 45, 15, 15),
            BlockInfo::new(67, 46, 16, 15),
        ]),

        // Version 40
        (40, ECLevel::LOW) => Ok(vec![
            BlockInfo::new(19, 148, 118, 15),
            BlockInfo::new(6, 149, 119, 15),
        ]),
        (40, ECLevel::MEDIUM) => Ok(vec![
            BlockInfo::new(18, 75, 47, 14),
            BlockInfo::new(31, 76, 48, 14),
        ]),
        (40, ECLevel::QUARTILE) => Ok(vec![
            BlockInfo::new(34, 54, 24, 15),
            BlockInfo::new(34, 55, 25, 15),
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

    for bi in &block_info {
        for _ in 0..bi.block_count {
            bi_unwound.push(bi.clone());
        }
    }

    Ok(bi_unwound)
}
