use crate::core::constants::ssh::{SCP_PERCENT, SCP_SIZE_UNITS, SCP_UNIT_BASE, SCP_UNIT_SUFFIX};
use crate::features::operations::domain::entities::scp_progress::ScpProgress;

pub fn scp_progress(line: &str) -> Option<ScpProgress> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let at = tokens.iter().position(|token| is_percent(token))?;
    let bytes = scp_bytes(tokens.get(at + 1)?)?;
    Some(ScpProgress {
        name: tokens[..at].join(" "),
        bytes,
    })
}

fn is_percent(token: &str) -> bool {
    token.strip_suffix(SCP_PERCENT).is_some_and(|digits| {
        !digits.is_empty() && digits.chars().all(|found| found.is_ascii_digit())
    })
}

fn scp_bytes(token: &str) -> Option<u64> {
    let split = token
        .find(|found: char| !found.is_ascii_digit())
        .unwrap_or(token.len());
    let (digits, unit) = token.split_at(split);
    let amount: u64 = digits.parse().ok()?;
    let unit = unit.trim_end_matches(SCP_UNIT_SUFFIX);
    if unit.is_empty() {
        return Some(amount);
    }
    let power = SCP_SIZE_UNITS.find(unit)? as u32 + 1;
    Some(amount.saturating_mul(SCP_UNIT_BASE.pow(power)))
}
