#![allow(dead_code)]

pub fn bytes_for(nbits: usize) -> usize {
    (nbits + 7) / 8
}

pub fn needed_bits(x: u64) -> usize {
    msb(x) + 1
}

pub fn msb(mut x: u64) -> usize {
    if x == 0 {
        return 0;
    }
    // right-saturate the word
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x |= x >> 32;
    // isolate the MSB
    x ^= x >> 1;
    bit_position(x)
}

fn bit_position(x: u64) -> usize {
    DEBRUIJN64_MAPPING[(DEBRUIJN64.wrapping_mul(x) >> 58) as usize] as usize
}

const DEBRUIJN64_MAPPING: [u8; 64] = [
    63, 0, 58, 1, 59, 47, 53, 2, 60, 39, 48, 27, 54, 33, 42, 3, 61, 51, 37, 40, 49, 18, 28, 20, 55,
    30, 34, 11, 43, 14, 22, 4, 62, 57, 46, 52, 38, 26, 32, 41, 50, 36, 17, 19, 29, 10, 13, 21, 56,
    45, 25, 31, 35, 16, 9, 12, 44, 24, 15, 8, 23, 7, 6, 5,
];

const DEBRUIJN64: u64 = 0x07EDD5E59A4E28C2;
