use crate::features::devices::domain::entities::format_request::{Filesystem, FormatError};

/// Apply the UI's conservative naming rules before any destructive operation.
pub fn volume_label(filesystem: Filesystem, input: &str) -> Result<String, FormatError> {
    if input.chars().any(char::is_control) {
        return Err(FormatError::InvalidLabel);
    }
    let label = input.trim();
    let valid = match filesystem {
        Filesystem::Fat32 => {
            label.is_ascii()
                && label.len() <= 11
                && !label.contains([
                    '"', '*', '+', ',', '.', '/', ':', ';', '<', '=', '>', '?', '[', '\\', ']', '|',
                ])
        }
        Filesystem::Exfat => {
            label.encode_utf16().count() <= 15
                && !label.contains(['"', '*', '/', ':', '<', '>', '?', '\\', '|'])
        }
        Filesystem::Ntfs => {
            label.encode_utf16().count() <= 128
                && !label.contains(['"', '*', '/', ':', '<', '>', '?', '\\', '|'])
        }
        Filesystem::Ext4 => label.len() <= 16,
    };
    if !valid {
        return Err(FormatError::InvalidLabel);
    }
    Ok(if filesystem == Filesystem::Fat32 {
        label.to_ascii_uppercase()
    } else {
        label.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_names_are_allowed_and_fat_names_are_normalized() {
        for filesystem in Filesystem::ALL {
            assert_eq!(volume_label(filesystem, "  "), Ok(String::new()));
        }
        assert_eq!(
            volume_label(Filesystem::Fat32, " mes_docs "),
            Ok("MES_DOCS".into())
        );
    }

    #[test]
    fn rejects_forbidden_characters_without_truncating() {
        for filesystem in [Filesystem::Fat32, Filesystem::Exfat, Filesystem::Ntfs] {
            assert_eq!(
                volume_label(filesystem, "bad/name"),
                Err(FormatError::InvalidLabel)
            );
        }
        for filesystem in Filesystem::ALL {
            assert_eq!(
                volume_label(filesystem, "bad\0name"),
                Err(FormatError::InvalidLabel)
            );
            assert_eq!(
                volume_label(filesystem, "bad\nname"),
                Err(FormatError::InvalidLabel)
            );
        }
        assert!(volume_label(Filesystem::Fat32, "été").is_err());
    }

    #[test]
    fn respects_byte_and_utf16_boundaries_including_non_bmp_characters() {
        assert!(volume_label(Filesystem::Fat32, &"a".repeat(11)).is_ok());
        assert!(volume_label(Filesystem::Fat32, &"a".repeat(12)).is_err());
        assert!(volume_label(Filesystem::Ext4, &"é".repeat(8)).is_ok());
        assert!(volume_label(Filesystem::Ext4, &"é".repeat(9)).is_err());
        for (filesystem, limit) in [(Filesystem::Exfat, 15), (Filesystem::Ntfs, 128)] {
            assert!(volume_label(filesystem, &"a".repeat(limit)).is_ok());
            assert!(volume_label(filesystem, &"a".repeat(limit + 1)).is_err());
            assert!(volume_label(filesystem, &"😀".repeat(limit / 2 + 1)).is_err());
        }
    }
}
