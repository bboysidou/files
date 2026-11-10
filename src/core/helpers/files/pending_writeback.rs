use std::fs;

use crate::core::constants::devices::{
    MEMINFO_DIRTY, MEMINFO_UNIT_BYTES, MEMINFO_WRITEBACK, PROC_MEMINFO,
};

pub fn pending_writeback() -> u64 {
    let Ok(text) = fs::read_to_string(PROC_MEMINFO) else {
        return 0;
    };
    text.lines()
        .filter(|line| line.starts_with(MEMINFO_DIRTY) || line.starts_with(MEMINFO_WRITEBACK))
        .filter_map(|line| line.split_whitespace().nth(1)?.parse::<u64>().ok())
        .sum::<u64>()
        * MEMINFO_UNIT_BYTES
}
