use crate::features::devices::domain::entities::format_request::{
    Filesystem, FormatError, FormatRequest, FormatTarget,
};

/// Implemented by the system adapter; tests never need access to a real disk.
pub trait FormatSession {
    fn inspect(&self, target: &FormatTarget) -> Result<FormatTarget, FormatError>;
    fn available(&self, filesystem: Filesystem) -> Result<bool, FormatError>;
    fn mounted(&self, target: &FormatTarget) -> Result<bool, FormatError>;
    fn unmount(&self, target: &FormatTarget) -> Result<(), FormatError>;
    fn create(&self, request: &FormatRequest) -> Result<(), FormatError>;
}

pub fn format_volume(
    session: &impl FormatSession,
    request: &FormatRequest,
) -> Result<(), FormatError> {
    let request = FormatRequest {
        label: super::volume_label::volume_label(request.filesystem, &request.label)?,
        ..request.clone()
    };
    let (target, filesystem) = (&request.target, request.filesystem);
    validate(target, &session.inspect(target)?)?;
    if !session.available(filesystem)? {
        return Err(FormatError::Unavailable);
    }
    if session.mounted(target)? {
        session.unmount(target)?;
    }
    validate(target, &session.inspect(target)?)?;
    session.create(&request)
}

fn validate(expected: &FormatTarget, current: &FormatTarget) -> Result<(), FormatError> {
    if expected == current {
        Ok(())
    } else {
        Err(FormatError::Changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn target() -> FormatTarget {
        FormatTarget {
            block: "/dev/sdb1".into(),
            object_path: "/block/sdb1".into(),
            disk_sequence: 3,
            device_number: 2049,
            size: 1024,
            uuid: "uuid".into(),
            drive_id: "usb".into(),
        }
    }

    fn request(filesystem: Filesystem) -> FormatRequest {
        FormatRequest {
            target: target(),
            filesystem,
            label: String::new(),
        }
    }

    #[test]
    fn invalid_label_is_rejected_before_inspection_or_unmount() {
        let session = Session::default();
        let mut request = request(Filesystem::Fat32);
        request.label = "name-too-long".into();
        assert_eq!(
            format_volume(&session, &request),
            Err(FormatError::InvalidLabel)
        );
        assert!(session.calls.borrow().is_empty());
    }

    struct Session {
        calls: RefCell<Vec<&'static str>>,
        changed_on: usize,
        available: bool,
        mounted: bool,
        unmount: Result<(), FormatError>,
        create: Result<(), FormatError>,
        label: RefCell<Option<String>>,
    }

    impl Default for Session {
        fn default() -> Self {
            Self {
                calls: RefCell::new(Vec::new()),
                changed_on: 0,
                available: true,
                mounted: true,
                unmount: Ok(()),
                create: Ok(()),
                label: RefCell::new(None),
            }
        }
    }

    impl FormatSession for Session {
        fn inspect(&self, _: &FormatTarget) -> Result<FormatTarget, FormatError> {
            self.calls.borrow_mut().push("inspect");
            let count = self
                .calls
                .borrow()
                .iter()
                .filter(|call| **call == "inspect")
                .count();
            let mut current = target();
            if count == self.changed_on {
                current.disk_sequence += 1;
            }
            Ok(current)
        }
        fn available(&self, _: Filesystem) -> Result<bool, FormatError> {
            self.calls.borrow_mut().push("available");
            Ok(self.available)
        }
        fn mounted(&self, _: &FormatTarget) -> Result<bool, FormatError> {
            self.calls.borrow_mut().push("mounted");
            Ok(self.mounted)
        }
        fn unmount(&self, _: &FormatTarget) -> Result<(), FormatError> {
            self.calls.borrow_mut().push("unmount");
            self.unmount
        }
        fn create(&self, request: &FormatRequest) -> Result<(), FormatError> {
            self.label.replace(Some(request.label.clone()));
            self.calls.borrow_mut().push("format");
            self.create
        }
    }

    #[test]
    fn normalizes_the_label_before_passing_it_to_the_system_adapter() {
        let session = Session::default();
        let mut request = request(Filesystem::Fat32);
        request.label = " mes_docs ".into();
        assert_eq!(format_volume(&session, &request), Ok(()));
        assert_eq!(*session.label.borrow(), Some("MES_DOCS".into()));
    }

    #[test]
    fn checks_identity_and_support_then_unmounts_and_checks_again_before_formatting() {
        let session = Session::default();
        assert_eq!(format_volume(&session, &request(Filesystem::Exfat)), Ok(()));
        assert_eq!(
            *session.calls.borrow(),
            [
                "inspect",
                "available",
                "mounted",
                "unmount",
                "inspect",
                "format"
            ]
        );
    }

    #[test]
    fn does_not_unmount_a_volume_that_is_already_unmounted() {
        let session = Session {
            mounted: false,
            ..Session::default()
        };
        assert_eq!(format_volume(&session, &request(Filesystem::Ext4)), Ok(()));
        assert!(!session.calls.borrow().contains(&"unmount"));
    }

    #[test]
    fn never_formats_a_device_replaced_before_or_during_unmount() {
        for changed_on in [1, 2] {
            let session = Session {
                changed_on,
                ..Session::default()
            };
            assert_eq!(
                format_volume(&session, &request(Filesystem::Fat32)),
                Err(FormatError::Changed)
            );
            assert!(!session.calls.borrow().contains(&"format"));
        }
    }

    #[test]
    fn missing_tool_does_not_unmount_or_format() {
        let session = Session {
            available: false,
            ..Session::default()
        };
        assert_eq!(
            format_volume(&session, &request(Filesystem::Ntfs)),
            Err(FormatError::Unavailable)
        );
        assert_eq!(*session.calls.borrow(), ["inspect", "available"]);
    }

    #[test]
    fn busy_or_refused_unmount_never_formats() {
        for error in [
            FormatError::Busy,
            FormatError::NotAuthorized,
            FormatError::Failed,
        ] {
            let session = Session {
                unmount: Err(error),
                ..Session::default()
            };
            assert_eq!(
                format_volume(&session, &request(Filesystem::Exfat)),
                Err(error)
            );
            assert!(!session.calls.borrow().contains(&"format"));
        }
    }

    #[test]
    fn propagates_format_failure_without_retrying() {
        let session = Session {
            create: Err(FormatError::Failed),
            ..Session::default()
        };
        assert_eq!(
            format_volume(&session, &request(Filesystem::Ext4)),
            Err(FormatError::Failed)
        );
        assert_eq!(
            session
                .calls
                .borrow()
                .iter()
                .filter(|call| **call == "format")
                .count(),
            1
        );
    }

    #[test]
    fn rejects_each_changed_identity_field() {
        let expected = target();
        let mut variants = vec![expected.clone(); 7];
        variants[0].block = "/dev/sdc1".into();
        variants[1].object_path = "/block/sdc1".into();
        variants[2].device_number += 1;
        variants[3].size += 1;
        variants[4].uuid = "other".into();
        variants[5].drive_id = "other".into();
        variants[6].disk_sequence += 1;
        for current in variants {
            assert_eq!(validate(&expected, &current), Err(FormatError::Changed));
        }
        assert_eq!(validate(&expected, &expected), Ok(()));
    }
}
