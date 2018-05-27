use qr::block_info;
use qr::format::ECLevel;
use qr::{BlockInfo, QRData, QRError};

use algorithm::decode::galois::{EXP8, GF8};

use std::ops::{Div, Mul, Sub};

pub fn correct(
    mut blocks: Vec<Vec<u8>>,
    data: &QRData,
    level: &ECLevel,
) -> Result<Vec<Vec<u8>>, QRError> {
    let block_info = &block_info(data.version, level)?[0];

    let mut corrected = vec![];

    let mut syndromes = vec![GF8(0); (block_info.ec_cap * 2) as usize];

    syndromes[0] = syndrome(&blocks[0], EXP8[0]);

    if syndromes[0] == GF8(0) {
        // all fine, nothing to do
        debug!("SYNDROME WAS ZERO, NO CORRECTION NEEDED");
        corrected.push(blocks[0].clone());
        return Ok(corrected);
    }

    for i in 1..block_info.ec_cap * 2 {
        syndromes[i as usize] = syndrome(&blocks[0], EXP8[i as usize]);
    }

    let locs = find_locs(block_info, &syndromes)?;

    let mut eq = vec![vec![GF8(0); locs.len() + 1]; locs.len()];
    for i in 0..locs.len() {
        for j in 0..locs.len() {
            eq[i][j] = EXP8[(i * locs[j] as usize) % 255];
        }

        eq[i][locs.len()] = syndromes[i];
    }

    let distance = solve(eq, GF8(0), GF8(1));

    let distance = distance.ok_or(QRError {
        msg: String::from("Could calculate error distances"),
    })?;

    for i in 0..locs.len() {
        debug!(
            "FIXING LOCATION {} FROM {:08b} TO {:08b}",
            block_info.total_per as usize - 1 - locs[i] as usize,
            blocks[0][block_info.total_per as usize - 1 - locs[i] as usize],
            blocks[0][block_info.total_per as usize - 1 - locs[i] as usize] ^ distance[i].0
        );
        blocks[0][block_info.total_per as usize - 1 - locs[i] as usize] ^= distance[i].0;
    }

    if syndrome(&blocks[0], EXP8[0]) != GF8(0) {
        return Err(QRError {
            msg: String::from("Error correcting did not fix corrupted data"),
        });
    }

    corrected.push(blocks[0].clone());

    Ok(corrected)
}

fn syndrome(block: &Vec<u8>, base: GF8) -> GF8 {
    let mut synd = GF8(0);
    let mut alpha = GF8(1);

    for codeword in block.iter().rev() {
        synd = synd + (alpha * GF8(*codeword));

        alpha = alpha * base;
    }

    synd
}

fn find_locs(block_info: &BlockInfo, syndromes: &Vec<GF8>) -> Result<Vec<usize>, QRError> {
    let mut sigma: Option<Vec<GF8>> = None;

    for z in (1..block_info.ec_cap as usize + 1).rev() {
        let mut eq = vec![vec![GF8(0); z + 1]; z];
        for i in 0..z {
            for j in 0..z + 1 {
                eq[i][j] = syndromes[i + j];
            }
        }

        sigma = solve(eq, GF8(0), GF8(1));

        if sigma.is_some() {
            break;
        }
    }

    let sigma = sigma.ok_or(QRError {
        msg: String::from("Could not calculate SIGMA"),
    })?;

    let mut locs = vec![];

    for i in 0..255 {
        let x_orig = EXP8[i];

        let mut x = x_orig;
        let mut check_value = sigma[0];
        for i in 1..sigma.len() {
            check_value = check_value + x * sigma[i];
            x = x * x_orig;
        }
        check_value = check_value + x;

        if check_value == GF8(0) {
            let loc = i as usize;

            if loc < block_info.total_per as usize {
                locs.push(loc);
            }
        }
    }

    debug!("LOCS {:?}", locs);

    Ok(locs)
}

use std::fmt::Debug;

fn solve<T>(mut eq: Vec<Vec<T>>, zero: T, one: T) -> Option<Vec<T>>
where
    T: Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy + PartialEq + Debug,
{
    let num_eq = eq.len() as usize;
    if num_eq == 0 {
        return None;
    }

    let num_coeff = eq[0].len();
    if num_coeff == 0 {
        return None;
    }

    for i in 0..num_eq {
        // normalise equation
        for j in (i..num_coeff).rev() {
            // divide all coefficients by the first nonzero
            // the first nonzero will now be GF8(1)
            eq[i][j] = eq[i][j] / eq[i][i];
        }

        // subtract normalised equation from others, multiplied by first coefficient
        // so the coefficients corresponding to the GF8(1) above will be GF8(0)
        for j in i + 1..num_eq {
            for k in (i..num_coeff).rev() {
                eq[j][k] = eq[j][k] - (eq[j][i] * eq[i][k]);
            }
        }

        // If the rank is too low, can't solve
        if eq[i][num_coeff - 1] == one {
            return None;
        }
    }

    let mut solution = vec![zero; num_eq];

    for i in (0..num_eq).rev() {
        solution[i] = eq[i][num_coeff - 1];
        for j in i + 1..num_coeff - 1 {
            solution[i] = solution[i] - (eq[i][j] * solution[j]);
        }
    }

    Some(solution)
}
