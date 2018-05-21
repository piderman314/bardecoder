use chomp::Chomp;

use qr::QRError;

pub fn data(input: Vec<u8>, version: u32) -> Result<String, QRError> {
    let mut chomp = Chomp::new(input);
    let mut result = String::new();

    loop {
        if let Some(mode) = chomp.chomp(4) {
            match mode {
                0b0001 => result.push_str(numeric(&mut chomp, version)?.as_str()),
                0b0000 => break,
                _ => {
                    return Err(QRError {
                        msg: format!("Mode {:04b} not yet implemented.", mode),
                    })
                }
            }
        } else {
            break;
        }
    }

    Ok(result)
}

fn numeric(chomp: &mut Chomp, version: u32) -> Result<String, QRError> {
    let length_bits = match version {
        01...09 => 10,
        10...26 => 12,
        27...40 => 14,
        _ => {
            return Err(QRError {
                msg: format!("Unknown version {}", version),
            });
        }
    };

    let mut length = chomp.chomp_or_u16(
        length_bits,
        QRError {
            msg: format!("Could not read {} bits for numeric length", length_bits),
        },
    )?;

    let mut result = String::new();

    while length > 0 {
        if length >= 3 {
            let digits = read_digits(chomp, 10)?;
            result.push_str(&format!("{:03}", digits));

            length -= 3;
            continue;
        }

        if length == 2 {
            let digits = read_digits(chomp, 7)?;
            result.push_str(&format!("{:02}", digits));

            break;
        }

        if length == 2 {
            let digits = read_digits(chomp, 4)?;
            result.push_str(&format!("{:01}", digits));

            break;
        }
    }

    Ok(result)
}

fn read_digits(chomp: &mut Chomp, bits: u8) -> Result<u16, QRError> {
    chomp.chomp_or_u16(
        bits,
        QRError {
            msg: format!("Could not read {} bits for numeric digits", bits),
        },
    )
}
