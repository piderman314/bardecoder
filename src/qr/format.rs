use algorithm::decode::galois::{EXP4, GF4, LOG4};
use qr::{QRData, QRError};

const MASK: [u8; 15] = [1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0];

pub enum ErrorCorrection {
    LOW,
    MEDIUM,
    QUARTILE,
    HIGH,
}

pub type QRMask = Fn(&QRData, u32, u32) -> u8;

pub fn format(data: &QRData) -> Result<(ErrorCorrection, Box<QRMask>), QRError> {
    let format1 = format1(data);

    Err(QRError {
        msg: String::from("Help!"),
    })
}

fn format1(data: &QRData) -> Result<Vec<u8>, QRError> {
    let mut format1 = vec![];

    for x in 0..9 {
        if x == 6 {
            continue;
        }

        format1.push(if data[[x, 8]] == 0 { 1 } else { 0 });
    }

    for y in (0..8).rev() {
        if y == 6 {
            continue;
        }

        format1.push(if data[[8, y]] == 0 { 1 } else { 0 });
    }

    for i in 0..format1.len() {
        format1[i] ^= MASK[i];
    }

    correct(format1)
}

fn format2(data: &QRData) -> Result<Vec<u8>, QRError> {
    let mut format2 = vec![];

    for y in (data.side - 7..data.side).rev() {
        format2.push(if data[[8, y]] == 0 { 1 } else { 0 });
    }

    for x in data.side - 8..data.side {
        format2.push(if data[[x, 8]] == 0 { 1 } else { 0 });
    }

    for i in 0..format2.len() {
        format2[i] ^= MASK[i];
    }

    correct(format2)
}

fn correct(mut format: Vec<u8>) -> Result<Vec<u8>, QRError> {
    let mut s1 = GF4(0);

    for i in 0..format.len() {
        s1 = s1 + GF4(format[format.len() - i - 1]) * EXP4[i % 15];
    }

    if s1 == GF4(0) {
        // syndrome == 0, no error detected
        return Ok(format);
    }

    let s2 = s1 * s1;
    let s4 = s2 * s2;

    let mut s3 = GF4(0);
    let mut s5 = GF4(0);

    for i in 0..format.len() {
        s3 = s3 + GF4(format[format.len() - i - 1]) * EXP4[(3 * i) % 15];
        s5 = s5 + GF4(format[format.len() - i - 1]) * EXP4[(5 * i) % 15];
    }

    let sigma1 = s1;
    let sigma2 = ((s5 + s4 * sigma1) - s2 * (s3 + s2 * sigma1)) / (s3 - s1 * s2);
    let sigma3 = s3 + s2 * sigma1 + s1 * sigma2;

    let mut error_pos = vec![];

    for i in 0..16 {
        let x = GF4(i);
        if sigma3 + sigma2 * x + sigma1 * x * x + x * x * x == GF4(0) {
            let log = LOG4[i as usize];
            if log != 0 {
                error_pos.push(log);
            }
        }
    }

    for error in error_pos {
        let len = format.len();
        format[len - error as usize - 1] ^= 1;
    }

    s1 = GF4(0);

    for i in 0..format.len() {
        s1 = s1 + GF4(format[format.len() - i - 1]) * EXP4[i % 15];
    }

    if s1 == GF4(0) {
        // syndrome == 0, no error detected
        return Ok(format);
    }

    Err(QRError {
        msg: String::from("Format information corrupted"),
    })
}

fn error_correction(bytes: u8) -> Option<ErrorCorrection> {
    match bytes {
        0b01 => Some(ErrorCorrection::LOW),
        0b00 => Some(ErrorCorrection::MEDIUM),
        0b11 => Some(ErrorCorrection::QUARTILE),
        0b10 => Some(ErrorCorrection::HIGH),
        _ => None,
    }
}

fn mask(bytes: u8) -> Option<Box<QRMask>> {
    match bytes {
        0b000 => qrmask(Box::new(|i, j| (i + j) % 2 == 0)),
        0b001 => qrmask(Box::new(|i, _| i % 2 == 0)),
        0b010 => qrmask(Box::new(|_, j| j % 2 == 0)),
        0b011 => qrmask(Box::new(|i, j| (i + j) % 3 == 0)),
        0b100 => qrmask(Box::new(|i, j| (i / 2 + j / 3) % 2 == 0)),
        0b101 => qrmask(Box::new(|i, j| (i * j) % 2 + (i * j) % 3 == 0)),
        0b110 => qrmask(Box::new(|i, j| ((i * j) % 2 + (i * j) % 3) % 2 == 0)),
        0b111 => qrmask(Box::new(|i, j| ((i * j) % 3 + (i + j) % 2) % 2 == 0)),
        _ => None,
    }
}

type Mask = Fn(u32, u32) -> bool;

fn qrmask(mask: Box<Mask>) -> Option<Box<QRMask>> {
    Some(Box::new(move |q: &QRData, i: u32, j: u32| {
        let d = q[[i, j]];

        if i <= 8 || j <= 8 || i >= q.side - 8 || j >= q.side - 8 {
            return d;
        }

        d ^ (if mask(i, j) { 1 } else { 0 })
    }))
}

#[cfg(test)]
mod test {
    use super::*;

    const CORRECT: [u8; 15] = [0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1];

    #[test]
    pub fn test_correct() {
        let mut input = CORRECT.to_vec();
        let input_to_check = input.clone();

        let output = correct(input);
        assert!(output.is_ok());
        assert_eq!(input_to_check, output.unwrap());
    }

    #[test]
    pub fn test_fixable() {
        let mut input_orig = CORRECT.to_vec();
        let mut input_fixable = input_orig.clone();
        input_fixable[4] ^= 1;
        input_fixable[12] ^= 1;

        let output = correct(input_fixable);
        assert!(output.is_ok());
        assert_eq!(input_orig, output.unwrap());
    }

    #[test]
    pub fn test_corrupt() {
        let mut input_orig = CORRECT.to_vec();
        let mut input_corrupt = input_orig.clone();
        input_corrupt[4] ^= 1;
        input_corrupt[5] ^= 1;
        input_corrupt[6] ^= 1;
        input_corrupt[12] ^= 1;
        input_corrupt[13] ^= 1;

        let output = correct(input_corrupt);

        println!("{:?}", output);

        assert!(output.is_err());
    }
}
