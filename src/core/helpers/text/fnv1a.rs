use crate::core::constants::hash::{FNV_OFFSET_BASIS, FNV_PRIME};

pub fn fnv1a(text: &str) -> u32 {
    text.bytes().fold(FNV_OFFSET_BASIS, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(FNV_PRIME)
    })
}
