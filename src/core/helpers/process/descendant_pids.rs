use std::fs;
use std::path::Path;

use crate::core::constants::sandbox::{PROC_CHILDREN_FILE, PROC_DIR, PROC_TASK_DIR};

pub fn descendant_pids(root: u32) -> Vec<u32> {
    let mut found = vec![root];
    let mut index = 0;
    while let Some(pid) = found.get(index).copied() {
        index += 1;
        let tasks = Path::new(PROC_DIR)
            .join(pid.to_string())
            .join(PROC_TASK_DIR);
        let Ok(tasks) = fs::read_dir(tasks) else {
            continue;
        };
        for task in tasks.flatten() {
            let Ok(children) = fs::read_to_string(task.path().join(PROC_CHILDREN_FILE)) else {
                continue;
            };
            found.extend(
                children
                    .split_whitespace()
                    .filter_map(|child| child.parse::<u32>().ok()),
            );
        }
    }
    found
}
