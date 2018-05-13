use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GF8(pub u8);

impl Add<GF8> for GF8 {
    type Output = GF8;

    fn add(self, other: GF8) -> GF8 {
        GF8(self.0 ^ other.0)
    }
}

impl Sub<GF8> for GF8 {
    type Output = GF8;

    fn sub(self, other: GF8) -> GF8 {
        GF8(self.0 ^ other.0)
    }
}

impl Mul<GF8> for GF8 {
    type Output = GF8;

    fn mul(self, other: GF8) -> GF8 {
        if self.0 == 0 || other.0 == 0 {
            return EXP8[255];
        }

        let log_self = LOG8[self.0 as usize];
        let log_other = LOG8[other.0 as usize];

        EXP8[((log_self as u16 + log_other as u16) % 255) as usize]
    }
}

impl Div<GF8> for GF8 {
    type Output = GF8;

    fn div(self, other: GF8) -> GF8 {
        let log_self = LOG8[self.0 as usize];
        let log_other = LOG8[other.0 as usize];
        let mut diff = log_self as i16 - log_other as i16;

        diff = if diff < 0 { diff + 255 } else { diff };

        EXP8[diff as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GF4(pub u8);

impl Add<GF4> for GF4 {
    type Output = GF4;

    fn add(self, other: GF4) -> GF4 {
        GF4(self.0 ^ other.0)
    }
}

impl Sub<GF4> for GF4 {
    type Output = GF4;

    fn sub(self, other: GF4) -> GF4 {
        GF4(self.0 ^ other.0)
    }
}

impl Mul<GF4> for GF4 {
    type Output = GF4;

    fn mul(self, other: GF4) -> GF4 {
        if self.0 == 0 || other.0 == 0 {
            return EXP4[15];
        }

        let log_self = LOG4[self.0 as usize];
        let log_other = LOG4[other.0 as usize];

        EXP4[((log_self as u16 + log_other as u16) % 15) as usize]
    }
}

impl Div<GF4> for GF4 {
    type Output = GF4;

    fn div(self, other: GF4) -> GF4 {
        let log_self = LOG4[self.0 as usize];
        let log_other = LOG4[other.0 as usize];
        let mut diff = log_self as i16 - log_other as i16;

        diff = if diff < 0 { diff + 15 } else { diff };

        EXP4[diff as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_gf8_add() {
        // zero
        assert_eq!(GF8(0) + GF8(123), GF8(123));

        // inverse sub
        assert_eq!(GF8(40) + GF8(193), GF8(233));

        // commutativity
        assert_eq!(GF8(40) + GF8(193), GF8(193) + GF8(40));

        // associativity
        assert_eq!(
            (GF8(40) + GF8(193)) + GF8(78),
            GF8(40) + (GF8(193) + GF8(78))
        );
    }

    #[test]
    pub fn test_gf8_sub() {
        // zero
        assert_eq!(GF8(123) - GF8(123), GF8(0));

        // inverse add
        assert_eq!(GF8(233) - GF8(193), GF8(40));
        assert_eq!(GF8(233) - GF8(40), GF8(193));
    }

    #[test]
    pub fn test_gf8_mul() {
        // zero
        assert_eq!(GF8(40) * GF8(0), GF8(0));
        assert_eq!(GF8(0) * GF8(40), GF8(0));

        // unit
        assert_eq!(GF8(40) * GF8(1), GF8(40));
        assert_eq!(GF8(1) * GF8(40), GF8(40));

        // inverse div
        assert_eq!(GF8(40) * GF8(193), GF8(67));

        // commutativity
        assert_eq!(GF8(40) * GF8(193), GF8(193) * GF8(40));

        // associativity
        assert_eq!(
            (GF8(40) * GF8(193)) * GF8(78),
            GF8(40) * (GF8(193) * GF8(78))
        );

        // distributivity
        assert_eq!(
            GF8(40) * (GF8(193) + GF8(78)),
            GF8(40) * GF8(193) + GF8(40) * GF8(78)
        );
    }

    #[test]
    pub fn test_gf8_div() {
        // unit
        assert_eq!(GF8(40) / GF8(40), GF8(1));
        assert_eq!(GF8(40) / GF8(1), GF8(40));

        // inverse mul
        assert_eq!(GF8(67) / GF8(193), GF8(40));
        assert_eq!(GF8(67) / GF8(40), GF8(193));
    }

    #[test]
    pub fn test_gf4_add() {
        // zero
        assert_eq!(GF4(0) + GF4(5), GF4(5));

        // inverse sub
        assert_eq!(GF4(3) + GF4(7), GF4(4));

        // commutativity
        assert_eq!(GF4(5) + GF4(9), GF4(9) + GF4(5));

        // associativity
        assert_eq!((GF4(3) + GF4(9)) + GF4(10), GF4(3) + (GF4(9) + GF4(10)));
    }

    #[test]
    pub fn test_gf4_sub() {
        // zero
        assert_eq!(GF4(5) - GF4(5), GF4(0));

        // inverse add
        assert_eq!(GF4(4) - GF4(3), GF4(7));
        assert_eq!(GF4(4) - GF4(7), GF4(3));
    }

    #[test]
    pub fn test_gf4_mul() {
        // zero
        assert_eq!(GF4(4) * GF4(0), GF4(0));
        assert_eq!(GF4(0) * GF4(4), GF4(0));

        // unit
        assert_eq!(GF4(4) * GF4(1), GF4(4));
        assert_eq!(GF4(1) * GF4(4), GF4(4));

        // inverse div
        assert_eq!(GF4(7) * GF4(3), GF4(9));

        // commutativity
        assert_eq!(GF4(2) * GF4(9), GF4(9) * GF4(2));

        // associativity
        assert_eq!((GF4(2) * GF4(9)) * GF4(13), GF4(2) * (GF4(9) * GF4(13)));

        // distributivity
        assert_eq!(
            GF4(2) * (GF4(5) + GF4(11)),
            GF4(2) * GF4(5) + GF4(2) * GF4(11)
        );
    }

    #[test]
    pub fn test_gf4_div() {
        // unit
        assert_eq!(GF4(4) / GF4(4), GF4(1));
        assert_eq!(GF4(4) / GF4(1), GF4(4));

        // inverse mul
        assert_eq!(GF4(9) / GF4(7), GF4(3));
        assert_eq!(GF4(9) / GF4(3), GF4(7));
    }

}

// exp and log tables with base 2 in Galois Field 2^8 under modulo 0b100011101
// to generate:
/* 
let mut log: Vec<u8> = vec![0; 256];
let mut exp: Vec<u8> = vec![0; 256];
let modulo: u16 = 0b100011101;

let mut alpha: u16 = 1;
for i in 0..255 {
    exp[i] = (alpha & 0xff) as u8;
    log[alpha as usize] = i as u8;

    alpha *= 2;
    if alpha > 255 {
        alpha ^= modulo
    }
}
*/

pub const EXP8: [GF8; 256] = [
    GF8(0x01),
    GF8(0x02),
    GF8(0x04),
    GF8(0x08),
    GF8(0x10),
    GF8(0x20),
    GF8(0x40),
    GF8(0x80),
    GF8(0x1D),
    GF8(0x3A),
    GF8(0x74),
    GF8(0xE8),
    GF8(0xCD),
    GF8(0x87),
    GF8(0x13),
    GF8(0x26),
    GF8(0x4C),
    GF8(0x98),
    GF8(0x2D),
    GF8(0x5A),
    GF8(0xB4),
    GF8(0x75),
    GF8(0xEA),
    GF8(0xC9),
    GF8(0x8F),
    GF8(0x03),
    GF8(0x06),
    GF8(0x0C),
    GF8(0x18),
    GF8(0x30),
    GF8(0x60),
    GF8(0xC0),
    GF8(0x9D),
    GF8(0x27),
    GF8(0x4E),
    GF8(0x9C),
    GF8(0x25),
    GF8(0x4A),
    GF8(0x94),
    GF8(0x35),
    GF8(0x6A),
    GF8(0xD4),
    GF8(0xB5),
    GF8(0x77),
    GF8(0xEE),
    GF8(0xC1),
    GF8(0x9F),
    GF8(0x23),
    GF8(0x46),
    GF8(0x8C),
    GF8(0x05),
    GF8(0x0A),
    GF8(0x14),
    GF8(0x28),
    GF8(0x50),
    GF8(0xA0),
    GF8(0x5D),
    GF8(0xBA),
    GF8(0x69),
    GF8(0xD2),
    GF8(0xB9),
    GF8(0x6F),
    GF8(0xDE),
    GF8(0xA1),
    GF8(0x5F),
    GF8(0xBE),
    GF8(0x61),
    GF8(0xC2),
    GF8(0x99),
    GF8(0x2F),
    GF8(0x5E),
    GF8(0xBC),
    GF8(0x65),
    GF8(0xCA),
    GF8(0x89),
    GF8(0x0F),
    GF8(0x1E),
    GF8(0x3C),
    GF8(0x78),
    GF8(0xF0),
    GF8(0xFD),
    GF8(0xE7),
    GF8(0xD3),
    GF8(0xBB),
    GF8(0x6B),
    GF8(0xD6),
    GF8(0xB1),
    GF8(0x7F),
    GF8(0xFE),
    GF8(0xE1),
    GF8(0xDF),
    GF8(0xA3),
    GF8(0x5B),
    GF8(0xB6),
    GF8(0x71),
    GF8(0xE2),
    GF8(0xD9),
    GF8(0xAF),
    GF8(0x43),
    GF8(0x86),
    GF8(0x11),
    GF8(0x22),
    GF8(0x44),
    GF8(0x88),
    GF8(0x0D),
    GF8(0x1A),
    GF8(0x34),
    GF8(0x68),
    GF8(0xD0),
    GF8(0xBD),
    GF8(0x67),
    GF8(0xCE),
    GF8(0x81),
    GF8(0x1F),
    GF8(0x3E),
    GF8(0x7C),
    GF8(0xF8),
    GF8(0xED),
    GF8(0xC7),
    GF8(0x93),
    GF8(0x3B),
    GF8(0x76),
    GF8(0xEC),
    GF8(0xC5),
    GF8(0x97),
    GF8(0x33),
    GF8(0x66),
    GF8(0xCC),
    GF8(0x85),
    GF8(0x17),
    GF8(0x2E),
    GF8(0x5C),
    GF8(0xB8),
    GF8(0x6D),
    GF8(0xDA),
    GF8(0xA9),
    GF8(0x4F),
    GF8(0x9E),
    GF8(0x21),
    GF8(0x42),
    GF8(0x84),
    GF8(0x15),
    GF8(0x2A),
    GF8(0x54),
    GF8(0xA8),
    GF8(0x4D),
    GF8(0x9A),
    GF8(0x29),
    GF8(0x52),
    GF8(0xA4),
    GF8(0x55),
    GF8(0xAA),
    GF8(0x49),
    GF8(0x92),
    GF8(0x39),
    GF8(0x72),
    GF8(0xE4),
    GF8(0xD5),
    GF8(0xB7),
    GF8(0x73),
    GF8(0xE6),
    GF8(0xD1),
    GF8(0xBF),
    GF8(0x63),
    GF8(0xC6),
    GF8(0x91),
    GF8(0x3F),
    GF8(0x7E),
    GF8(0xFC),
    GF8(0xE5),
    GF8(0xD7),
    GF8(0xB3),
    GF8(0x7B),
    GF8(0xF6),
    GF8(0xF1),
    GF8(0xFF),
    GF8(0xE3),
    GF8(0xDB),
    GF8(0xAB),
    GF8(0x4B),
    GF8(0x96),
    GF8(0x31),
    GF8(0x62),
    GF8(0xC4),
    GF8(0x95),
    GF8(0x37),
    GF8(0x6E),
    GF8(0xDC),
    GF8(0xA5),
    GF8(0x57),
    GF8(0xAE),
    GF8(0x41),
    GF8(0x82),
    GF8(0x19),
    GF8(0x32),
    GF8(0x64),
    GF8(0xC8),
    GF8(0x8D),
    GF8(0x07),
    GF8(0x0E),
    GF8(0x1C),
    GF8(0x38),
    GF8(0x70),
    GF8(0xE0),
    GF8(0xDD),
    GF8(0xA7),
    GF8(0x53),
    GF8(0xA6),
    GF8(0x51),
    GF8(0xA2),
    GF8(0x59),
    GF8(0xB2),
    GF8(0x79),
    GF8(0xF2),
    GF8(0xF9),
    GF8(0xEF),
    GF8(0xC3),
    GF8(0x9B),
    GF8(0x2B),
    GF8(0x56),
    GF8(0xAC),
    GF8(0x45),
    GF8(0x8A),
    GF8(0x09),
    GF8(0x12),
    GF8(0x24),
    GF8(0x48),
    GF8(0x90),
    GF8(0x3D),
    GF8(0x7A),
    GF8(0xF4),
    GF8(0xF5),
    GF8(0xF7),
    GF8(0xF3),
    GF8(0xFB),
    GF8(0xEB),
    GF8(0xCB),
    GF8(0x8B),
    GF8(0x0B),
    GF8(0x16),
    GF8(0x2C),
    GF8(0x58),
    GF8(0xB0),
    GF8(0x7D),
    GF8(0xFA),
    GF8(0xE9),
    GF8(0xCF),
    GF8(0x83),
    GF8(0x1B),
    GF8(0x36),
    GF8(0x6C),
    GF8(0xD8),
    GF8(0xAD),
    GF8(0x47),
    GF8(0x8E),
    GF8(0x00),
];

pub const LOG8: [u8; 256] = [
    0x00, 0x00, 0x01, 0x19, 0x02, 0x32, 0x1A, 0xC6, 0x03, 0xDF, 0x33, 0xEE, 0x1B, 0x68, 0xC7, 0x4B,
    0x04, 0x64, 0xE0, 0x0E, 0x34, 0x8D, 0xEF, 0x81, 0x1C, 0xC1, 0x69, 0xF8, 0xC8, 0x08, 0x4C, 0x71,
    0x05, 0x8A, 0x65, 0x2F, 0xE1, 0x24, 0x0F, 0x21, 0x35, 0x93, 0x8E, 0xDA, 0xF0, 0x12, 0x82, 0x45,
    0x1D, 0xB5, 0xC2, 0x7D, 0x6A, 0x27, 0xF9, 0xB9, 0xC9, 0x9A, 0x09, 0x78, 0x4D, 0xE4, 0x72, 0xA6,
    0x06, 0xBF, 0x8B, 0x62, 0x66, 0xDD, 0x30, 0xFD, 0xE2, 0x98, 0x25, 0xB3, 0x10, 0x91, 0x22, 0x88,
    0x36, 0xD0, 0x94, 0xCE, 0x8F, 0x96, 0xDB, 0xBD, 0xF1, 0xD2, 0x13, 0x5C, 0x83, 0x38, 0x46, 0x40,
    0x1E, 0x42, 0xB6, 0xA3, 0xC3, 0x48, 0x7E, 0x6E, 0x6B, 0x3A, 0x28, 0x54, 0xFA, 0x85, 0xBA, 0x3D,
    0xCA, 0x5E, 0x9B, 0x9F, 0x0A, 0x15, 0x79, 0x2B, 0x4E, 0xD4, 0xE5, 0xAC, 0x73, 0xF3, 0xA7, 0x57,
    0x07, 0x70, 0xC0, 0xF7, 0x8C, 0x80, 0x63, 0x0D, 0x67, 0x4A, 0xDE, 0xED, 0x31, 0xC5, 0xFE, 0x18,
    0xE3, 0xA5, 0x99, 0x77, 0x26, 0xB8, 0xB4, 0x7C, 0x11, 0x44, 0x92, 0xD9, 0x23, 0x20, 0x89, 0x2E,
    0x37, 0x3F, 0xD1, 0x5B, 0x95, 0xBC, 0xCF, 0xCD, 0x90, 0x87, 0x97, 0xB2, 0xDC, 0xFC, 0xBE, 0x61,
    0xF2, 0x56, 0xD3, 0xAB, 0x14, 0x2A, 0x5D, 0x9E, 0x84, 0x3C, 0x39, 0x53, 0x47, 0x6D, 0x41, 0xA2,
    0x1F, 0x2D, 0x43, 0xD8, 0xB7, 0x7B, 0xA4, 0x76, 0xC4, 0x17, 0x49, 0xEC, 0x7F, 0x0C, 0x6F, 0xF6,
    0x6C, 0xA1, 0x3B, 0x52, 0x29, 0x9D, 0x55, 0xAA, 0xFB, 0x60, 0x86, 0xB1, 0xBB, 0xCC, 0x3E, 0x5A,
    0xCB, 0x59, 0x5F, 0xB0, 0x9C, 0xA9, 0xA0, 0x51, 0x0B, 0xF5, 0x16, 0xEB, 0x7A, 0x75, 0x2C, 0xD7,
    0x4F, 0xAE, 0xD5, 0xE9, 0xE6, 0xE7, 0xAD, 0xE8, 0x74, 0xD6, 0xF4, 0xEA, 0xA8, 0x50, 0x58, 0xAF,
];

pub const EXP4: [GF4; 16] = [
    GF4(0x01),
    GF4(0x02),
    GF4(0x04),
    GF4(0x08),
    GF4(0x03),
    GF4(0x06),
    GF4(0x0C),
    GF4(0x0B),
    GF4(0x05),
    GF4(0x0A),
    GF4(0x07),
    GF4(0x0E),
    GF4(0x0F),
    GF4(0x0D),
    GF4(0x09),
    GF4(0x00),
];

pub const LOG4: [u8; 16] = [
    0x00, 0x00, 0x01, 0x04, 0x02, 0x08, 0x05, 0x0A, 0x03, 0x0E, 0x09, 0x07, 0x06, 0x0D, 0x0B, 0x0C,
];
