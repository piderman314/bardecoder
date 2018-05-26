use qr::format::QRMask;
use qr::{QRData, QRError};

pub fn blocks(data: &QRData, mask: Box<QRMask>) -> Result<Vec<Vec<u8>>, QRError> {
    let mut codewords = Codewords::new();
    let mut x = data.side - 1;
    let loc = alignment_location(data.version)?;

    loop {
        let y_range = y_range(x, data.side);

        for y in y_range {
            if is_data(data, &loc, x, y) {
                codewords.add_bit(mask(data, x, y));
            }

            if is_data(data, &loc, x - 1, y) {
                codewords.add_bit(mask(data, x - 1, y));
            }
        }

        if x == 1 {
            break;
        }

        x -= 2;
        if x == 6 {
            // skip timing pattern
            x = 5
        }
    }

    Ok(vec![codewords.codewords()])
}

fn y_range(x: u32, side: u32) -> Box<Iterator<Item = u32>> {
    if (x as i64 - side as i64 + 1) % 4 == 0 {
        Box::new((0..side).rev())
    } else {
        Box::new(0..side)
    }
}

fn is_data(data: &QRData, loc: &AlignmentLocation, x: u32, y: u32) -> bool {
    // timing patterns
    if x == 6 || y == 6 {
        return false;
    }

    // top left locator pattern
    if x < 9 && y < 9 {
        return false;
    }

    // top right locator pattern
    if x > data.side - 9 && y < 9 {
        return false;
    }

    // bottom left locator pattern
    if x < 9 && y > data.side - 9 {
        return false;
    }

    if is_alignment_coord(loc, x) && is_alignment_coord(loc, y) {
        return false;
    }

    true
}

fn is_alignment_coord(loc: &AlignmentLocation, coord: u32) -> bool {
    if coord - 4 % 6 <= 4 {
        return true;
    }

    if coord < loc.start - 2 {
        return false;
    }

    if (coord - loc.start + 2) % loc.step <= 4 {
        return true;
    }

    false
}

fn alignment_location(version: u32) -> Result<AlignmentLocation, QRError> {
    match version {
        // no alignment patterns for version 1 but this saves some exception paths
        1 => Ok(AlignmentLocation::new(1000, 1000)),

        // only one alignment pattern for versions 2-6 but this saves some exception paths
        2 => Ok(AlignmentLocation::new(18, 1000)),
        3 => Ok(AlignmentLocation::new(22, 1000)),
        4 => Ok(AlignmentLocation::new(26, 1000)),
        5 => Ok(AlignmentLocation::new(30, 1000)),
        6 => Ok(AlignmentLocation::new(34, 1000)),

        7 => Ok(AlignmentLocation::new(22, 16)),
        36 => Ok(AlignmentLocation::new(24, 26)),
        _ => Err(QRError {
            msg: format!("Unknown version {}", version),
        }),
    }
}

#[derive(Debug)]
struct AlignmentLocation {
    start: u32,
    step: u32,
}

impl AlignmentLocation {
    fn new(start: u32, step: u32) -> AlignmentLocation {
        AlignmentLocation { start, step }
    }
}

struct Codewords {
    codewords: Vec<u8>,
    current_byte: u8,
    bit_count: u8,
}

impl Codewords {
    fn new() -> Codewords {
        Codewords {
            codewords: vec![],
            current_byte: 0,
            bit_count: 0,
        }
    }

    fn add_bit(&mut self, bit: u8) {
        self.current_byte *= 2;
        self.current_byte += bit;
        self.bit_count += 1;

        if self.bit_count == 8 {
            self.codewords.push(self.current_byte);
            self.current_byte = 0;
            self.bit_count = 0;
        }
    }

    fn codewords(self) -> Vec<u8> {
        self.codewords
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_alignment_locs() {
        let al = alignment_location(36).unwrap();
        let side = 4 * 36 + 17;

        for x in 0..side {
            if (x >= 4 && x <= 8)
                || (x >= 22 && x <= 26)
                || (x >= 48 && x <= 52)
                || (x >= 74 && x <= 78)
                || (x >= 100 && x <= 104)
                || (x >= 126 && x <= 130)
                || (x >= 152 && x <= 156)
            {
                assert!(is_alignment_coord(&al, x));
            } else {
                assert!(!is_alignment_coord(&al, x));
            }
        }
    }
}
