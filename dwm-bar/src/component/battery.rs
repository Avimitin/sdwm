use super::widget::Block;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use dbus::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static::lazy_static!(
    /// Global DBus connection, can be shared between thread.
    static ref DBUS_SYSTEM: Arc<Mutex<Connection>> = Arc::new(
        Mutex::new(
            Connection::new_system()
            .expect(
                "Fail to connect to dbus, please ensure you are running in Linux with DBus, or ensure you have DBus daemon enabled."
            )
        ));
);

/// Enumerate through the devices, return path with pattern matched.
fn get_device_path(pat: &str) -> Option<Path> {
    let conn = DBUS_SYSTEM.lock().unwrap();
    let proxy = conn.with_proxy(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        Duration::from_millis(2000),
    );

    let (devices,): (Vec<Path>,) = proxy
        .method_call("org.freedesktop.UPower", "EnumerateDevices", ())
        .ok()?;

    let mut device = Path::default();
    for dev in devices {
        if dev.contains(pat) {
            device = dev;
            break;
        }
    }

    if device.is_empty() {
        None
    } else {
        Some(device)
    }
}

/// Build a component to show laptop battery percentage and power-supply status.
///
/// Return None if no battery device name contains "BAT0" keyword,
/// or PowerSupply/Percentage property are not found.
pub fn battery() -> Option<Block> {
    let device = get_device_path("BAT0")?;
    let conn = DBUS_SYSTEM.lock().unwrap();
    let proxy = conn.with_proxy(
        "org.freedesktop.UPower",
        device,
        Duration::from_millis(2000),
    );

    let has_power_supply: bool = proxy
        .get("org.freedesktop.UPower.Device", "PowerSupply")
        .ok()?;
    let percentage: f64 = proxy
        .get("org.freedesktop.UPower.Device", "Percentage")
        .ok()?;

    let icon = if has_power_supply { "" } else { "" };

    Some(
        Block::new(icon, format!("{:.0} %", percentage))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}

/// Build a headset battery component.
///
/// Return None if no battery device name contains "headset" keyword,
/// or no percentage property is found.
pub fn headset_battery() -> Option<Block> {
    let device = get_device_path("headset")?;
    let conn = DBUS_SYSTEM.lock().unwrap();
    let proxy = conn.with_proxy(
        "org.freedesktop.UPower",
        device,
        Duration::from_millis(2000),
    );

    let percentage: f64 = proxy
        .get("org.freedesktop.UPower.Device", "Percentage")
        .ok()?;

    Some(
        Block::new("", format!("{:.0}%", percentage))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}
