//! SumHash512 (https://github.com/algorand/go-sumhash): the subset-sum hash
//! Algorand uses for state proof commitments, with no salt.
//!
//! go-sumhash speeds compression up with a 2 MB lookup table; this uses the
//! 64 KB matrix directly, which suits the WASM sandbox's memory limits.

use alloc::vec;
use alloc::vec::Vec;

use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;

const ROWS: usize = 8; // n: output is 8 u64s = 64 bytes
const COLS: usize = 1024; // m: input is 1024 bits = 128 bytes
const OUTPUT_LEN: usize = ROWS * 8;
const INPUT_LEN: usize = COLS / 8;
/// Bytes of message per compression.
pub const BLOCK_LEN: usize = INPUT_LEN - OUTPUT_LEN;
pub const DIGEST_LEN: usize = OUTPUT_LEN;

/// The SumHash512 compression matrix: `RandomMatrixFromSeed("Algorand", 8, 1024)`.
pub struct Matrix(Vec<u64>);

impl Matrix {
    pub fn new() -> Self {
        let mut xof = Shake256::default();
        xof.update(&64u16.to_le_bytes());
        xof.update(&(ROWS as u16).to_le_bytes());
        xof.update(&(COLS as u16).to_le_bytes());
        xof.update(b"Algorand");
        let mut reader = xof.finalize_xof();
        let mut entries = vec![0u64; ROWS * COLS];
        let mut word = [0u8; 8];
        for entry in &mut entries {
            reader.read(&mut word);
            *entry = u64::from_le_bytes(word);
        }
        Matrix(entries)
    }

    fn compress(&self, dst: &mut [u8; OUTPUT_LEN], msg: &[u8; INPUT_LEN]) {
        for i in 0..ROWS {
            let row = &self.0[i * COLS..(i + 1) * COLS];
            let mut x = 0u64;
            for (j, byte) in msg.iter().enumerate() {
                for bit in 0..8 {
                    // Branch-free, as in go-sumhash.
                    let mask = 0u64.wrapping_sub(u64::from((byte >> bit) & 1));
                    x = x.wrapping_add(row[8 * j + bit] & mask);
                }
            }
            dst[8 * i..8 * i + 8].copy_from_slice(&x.to_le_bytes());
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

/// A SumHash512 computation in progress.
pub struct SumHash512<'m> {
    matrix: &'m Matrix,
    h: [u8; OUTPUT_LEN],
    x: [u8; BLOCK_LEN],
    nx: usize,
    len: u64,
}

impl<'m> SumHash512<'m> {
    pub fn new(matrix: &'m Matrix) -> Self {
        SumHash512 {
            matrix,
            h: [0; OUTPUT_LEN],
            x: [0; BLOCK_LEN],
            nx: 0,
            len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        self.len += data.len() as u64;
        if self.nx > 0 {
            let n = (BLOCK_LEN - self.nx).min(data.len());
            self.x[self.nx..self.nx + n].copy_from_slice(&data[..n]);
            self.nx += n;
            data = &data[n..];
            if self.nx == BLOCK_LEN {
                let block = self.x;
                self.block(&block);
                self.nx = 0;
            }
        }
        while data.len() >= BLOCK_LEN {
            let (block, rest) = data.split_at(BLOCK_LEN);
            self.block(block.try_into().unwrap());
            data = rest;
        }
        if !data.is_empty() {
            self.x[..data.len()].copy_from_slice(data);
            self.nx = data.len();
        }
    }

    fn block(&mut self, block: &[u8; BLOCK_LEN]) {
        let mut cin = [0u8; INPUT_LEN];
        cin[..OUTPUT_LEN].copy_from_slice(&self.h);
        cin[OUTPUT_LEN..].copy_from_slice(block);
        let mut out = [0u8; OUTPUT_LEN];
        self.matrix.compress(&mut out, &cin);
        self.h = out;
    }

    pub fn finalize(mut self) -> [u8; DIGEST_LEN] {
        let b = BLOCK_LEN as u64;
        let p = b - 16;
        let bit_len = self.len << 3;
        let mut pad = [0u8; BLOCK_LEN];
        pad[0] = 0x01;
        let r = self.len % b;
        let pad_len = if r < p { p - r } else { b + p - r } as usize;
        self.update(&pad[..pad_len]);
        let mut tail = [0u8; 16];
        tail[..8].copy_from_slice(&bit_len.to_le_bytes());
        self.update(&tail);
        debug_assert_eq!(self.nx, 0);
        self.h
    }
}

/// SumHash512 of `data`.
pub fn sumhash512(matrix: &Matrix, data: &[u8]) -> [u8; DIGEST_LEN] {
    let mut h = SumHash512::new(matrix);
    h.update(data);
    h.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8]) -> alloc::string::String {
        use core::fmt::Write;
        bytes.iter().fold(alloc::string::String::new(), |mut out, b| {
            let _ = write!(out, "{b:02x}");
            out
        })
    }

    /// go-sumhash's `TestSumHash512TestVector`.
    #[test]
    fn go_sumhash_known_answers() {
        let matrix = Matrix::new();
        let vectors = [
            ("", "591591c93181f8f90054d138d6fa85b63eeeb416e6fd201e8375ba05d3cb55391047b9b64e534042562cc61944930c0075f906f16710cdade381ee9dd47d10a0"),
            ("a", "ea067eb25622c633f5ead70ab83f1d1d76a7def8d140a587cb29068b63cb6407107aceecfdffa92579ed43db1eaa5bbeb4781223a6e07dd5b5a12d5e8bde82c6"),
            ("ab", "ef09d55b6add510f1706a52c4b45420a6945d0751d73b801cbc195a54bc0ade0c9ebe30e09c2c00864f2bd1692eba79500965925e2be2d1ac334425d8d343694"),
            ("abc", "a8e9b8259a93b8d2557434905790114a2a2e979fbdc8aa6fd373315a322bf0920a9b49f3dc3a744d8c255c46cd50ff196415c8245cdbb2899dec453fca2ba0f4"),
            ("abcd", "1d4277f17e522c4607bc2912bb0d0ac407e60e3c86e2b6c7daa99e1f740fe2b4fc928defad8e1ccc4e7d96b79896ffe086836c172a3db40a154d2229484f359b"),
            ("You must be the change you wish to see in the world. -Mahatma Gandhi", "5c5f63ac24392d640e5799c4164b7cc03593feeec85844cc9691ea0612a97caabc8775482624e1cd01fb8ce1eca82a17dd9d4b73e00af4c0468fd7d8e6c2e4b5"),
            ("I think, therefore I am. – Rene Descartes.", "2d4583cdb18710898c78ec6d696a86cc2a8b941bb4d512f9d46d96816d95cbe3f867c9b8bd31964406c847791f5669d60b603c9c4d69dadcb87578e613b60b7a"),
        ];
        for (input, expected) in vectors {
            assert_eq!(
                hex(&sumhash512(&matrix, input.as_bytes())),
                expected,
                "{input:?}"
            );
        }
    }

    #[test]
    fn incremental_updates_match_one_shot() {
        let matrix = Matrix::new();
        let data: Vec<u8> = (0..1000u32).map(|i| (i * 7) as u8).collect();
        for split in [0, 1, 63, 64, 65, 500, 999] {
            let mut h = SumHash512::new(&matrix);
            h.update(&data[..split]);
            h.update(&data[split..]);
            assert_eq!(h.finalize(), sumhash512(&matrix, &data), "split {split}");
        }
    }
}
