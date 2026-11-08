use crate::core::constants::hash::{FNV_OFFSET_BASIS, FNV_PRIME};

pub fn fnv1a_seeded(seed: u32, text: &str) -> u32 {
    let start = (FNV_OFFSET_BASIS ^ seed).wrapping_mul(FNV_PRIME);
    text.bytes().fold(start, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(FNV_PRIME)
    })
}
