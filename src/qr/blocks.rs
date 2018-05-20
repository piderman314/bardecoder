use qr::format::QRMask;
use qr::{QRData, QRError};

pub fn blocks(data: &QRData, mask: Box<QRMask>) -> Result<Vec<Vec<u8>>, QRError> {
    let mut codewords = Codewords::new();
    let mut x = data.side - 1;

    loop {
        let y_range = y_range(x, data.side);

        for y in y_range {
            if is_data(data, x, y) {
                codewords.add_bit(mask(data, x, y));
            }

            if is_data(data, x - 1, y) {
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

fn is_data(data: &QRData, x: u32, y: u32) -> bool {
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

    true
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
