use std::collections::HashMap;
use std::time::Duration;

use zbus::blocking::{Proxy, connection};
use zbus::zvariant::{OwnedObjectPath, OwnedValue};

type Properties = HashMap<String, OwnedValue>;
type Objects = HashMap<OwnedObjectPath, HashMap<String, Properties>>;
const SERVICE: &str = "org.freedesktop.UDisks2";
const JOB: &str = "org.freedesktop.UDisks2.Job";

/// Progress is optional: failure to observe a job must never fail formatting.
pub fn progress(target: &str) -> Option<f32> {
    let connection = connection::Builder::system()
        .ok()?
        .method_timeout(Duration::from_secs(2))
        .build()
        .ok()?;
    let manager = Proxy::new(
        &connection,
        SERVICE,
        "/org/freedesktop/UDisks2",
        "org.freedesktop.DBus.ObjectManager",
    )
    .ok()?;
    let objects: Objects = manager.call("GetManagedObjects", &()).ok()?;
    objects
        .values()
        .filter_map(|interfaces| job_progress(interfaces.get(JOB)?, target))
        .max_by(f32::total_cmp)
}

fn job_progress(properties: &Properties, target: &str) -> Option<f32> {
    let operation = <&str>::try_from(properties.get("Operation")?).ok()?;
    let uid = u32::try_from(properties.get("StartedByUID")?).ok()?;
    if operation != "format-mkfs" || uid != rustix::process::getuid().as_raw() {
        return None;
    }
    let objects: Vec<OwnedObjectPath> = properties
        .get("Objects")?
        .try_clone()
        .ok()?
        .try_into()
        .ok()?;
    if !objects.iter().any(|object| object.as_str() == target)
        || !bool::try_from(properties.get("ProgressValid")?).ok()?
    {
        return None;
    }
    let fraction = f64::try_from(properties.get("Progress")?).ok()?;
    (fraction.is_finite() && (0.0..=1.0).contains(&fraction)).then_some(fraction as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::zvariant::Value;

    fn job() -> Properties {
        HashMap::from([
            (
                "Operation".into(),
                Value::from("format-mkfs").try_into().unwrap(),
            ),
            (
                "StartedByUID".into(),
                OwnedValue::from(rustix::process::getuid().as_raw()),
            ),
            (
                "Objects".into(),
                Value::from(vec![OwnedObjectPath::try_from("/block/usb").unwrap()])
                    .try_into()
                    .unwrap(),
            ),
            ("ProgressValid".into(), OwnedValue::from(true)),
            ("Progress".into(), OwnedValue::from(0.42_f64)),
        ])
    }

    #[test]
    fn reads_only_valid_progress_for_this_volume_and_operation() {
        assert_eq!(job_progress(&job(), "/block/usb"), Some(0.42));
        assert_eq!(job_progress(&job(), "/block/other"), None);
        let mut properties = job();
        properties.insert(
            "Operation".into(),
            Value::from("filesystem-mount").try_into().unwrap(),
        );
        assert_eq!(job_progress(&properties, "/block/usb"), None);
        properties = job();
        properties.insert(
            "StartedByUID".into(),
            OwnedValue::from(rustix::process::getuid().as_raw().wrapping_add(1)),
        );
        assert_eq!(job_progress(&properties, "/block/usb"), None);
    }

    #[test]
    fn unavailable_invalid_and_non_finite_progress_stays_indeterminate() {
        for fraction in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
            let mut properties = job();
            properties.insert("Progress".into(), OwnedValue::from(fraction));
            assert_eq!(job_progress(&properties, "/block/usb"), None);
        }
        let mut properties = job();
        properties.insert("ProgressValid".into(), OwnedValue::from(false));
        assert_eq!(job_progress(&properties, "/block/usb"), None);
        assert_eq!(job_progress(&Properties::new(), "/block/usb"), None);
    }
}
