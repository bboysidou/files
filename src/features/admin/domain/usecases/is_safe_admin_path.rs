use std::path::{Component, Path};

pub fn is_safe_admin_path(path: &Path) -> bool {
    path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::RootDir | Component::Normal(_)))
}

#[cfg(test)]
mod tests {
    use super::is_safe_admin_path;
    use std::path::Path;

    #[test]
    fn rejects_relative_and_parent_paths() {
        assert!(is_safe_admin_path(Path::new("/etc/hosts")));
        assert!(!is_safe_admin_path(Path::new("etc/hosts")));
        assert!(!is_safe_admin_path(Path::new("/etc/../root")));
    }
}
