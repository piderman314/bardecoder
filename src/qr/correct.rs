use qr::block_info;
use qr::format::ECLevel;
use qr::{QRData, QRError};

use algorithm::decode::galois::{EXP8, GF8, LOG8};

use std::ops::{Div, Mul, Sub};

pub fn correct(
    mut blocks: Vec<Vec<u8>>,
    data: &QRData,
    level: ECLevel,
) -> Result<Vec<Vec<u8>>, QRError> {
    let block_info = &block_info(data.version, level)?[0];

    let mut corrected = vec![];

    let mut syndromes = vec![GF8(0); (block_info.ec_cap * 2) as usize];

    syndromes[0] = syndrome(&blocks[0], EXP8[0]);

    if syndromes[0] == GF8(0) {
        // all fine, nothing to do
        corrected.push(blocks[0].clone());
        return Ok(corrected);
    }

    for i in 1..block_info.ec_cap * 2 {
        syndromes[i as usize] = syndrome(&blocks[0], EXP8[i as usize]);
    }

    let mut eq = vec![vec![GF8(0); block_info.ec_cap as usize + 1]; block_info.ec_cap as usize];
    for i in 0..block_info.ec_cap as usize {
        for j in 0..5 {
            eq[i][j] = syndromes[i + j];
        }
    }

    let sigma = solve(eq, GF8(0));

    let mut locs = vec![];

    for i in 0..255 {
        let x = GF8(i);

        if sigma[0] + sigma[1] * x + sigma[2] * x * x + sigma[3] * x * x * x + x * x * x * x
            == GF8(0)
        {
            let loc = LOG8[i as usize];

            if (loc as usize) < blocks[0].len() {
                locs.push(loc);
            }
        }
    }

    let mut eq = vec![vec![GF8(0); locs.len() + 1]; locs.len()];
    for i in 0..locs.len() {
        for j in 0..locs.len() {
            eq[i][j] = EXP8[i * locs[j] as usize];
        }

        eq[i][locs.len()] = syndromes[i];
    }

    let distance = solve(eq, GF8(0));

    for i in 0..locs.len() {
        blocks[0][block_info.total_per as usize - 1 - locs[i] as usize] ^= distance[i].0;
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

fn solve<T>(mut eq: Vec<Vec<T>>, zero: T) -> Vec<T>
where
    T: Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
    let num_eq = eq.len() as usize;
    if num_eq == 0 {
        return vec![];
    }

    let num_coeff = eq[0].len();
    if num_coeff == 0 {
        return vec![];
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
    }

    let mut solution = vec![zero; num_eq];

    for i in (0..num_eq).rev() {
        solution[i] = eq[i][num_coeff - 1];
        for j in i + 1..num_coeff - 1 {
            solution[i] = solution[i] - (eq[i][j] * solution[j]);
        }
    }

    solution
}
