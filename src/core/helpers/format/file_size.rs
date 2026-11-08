use crate::core::constants::files::{SIZE_ONE_DECIMAL_BELOW, SIZE_STEP, SIZE_UNITS};

pub fn file_size(bytes: u64) -> String {
    let mut value = bytes as f64;
    if value < SIZE_STEP {
        return format!("{bytes} B");
    }
    let mut unit = 0;
    value /= SIZE_STEP;
    while value >= SIZE_STEP && unit < SIZE_UNITS.len() - 1 {
        value /= SIZE_STEP;
        unit += 1;
    }
    if value < SIZE_ONE_DECIMAL_BELOW {
        format!("{value:.1} {}", SIZE_UNITS[unit])
    } else {
        format!("{value:.0} {}", SIZE_UNITS[unit])
    }
}
