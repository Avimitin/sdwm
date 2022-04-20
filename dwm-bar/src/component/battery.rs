use super::widget::Block;
use tokio::fs;

/// Build a component to show laptop battery percentage and power-supply status.
/// Statistic come from /sys/class/power_supply/<bat_name>/{capacity,status}.
///
/// Return None if no battery device name contains "BAT0" keyword,
/// or no capacity/status file was found.
pub async fn battery() -> Option<Block> {
    let perc = tokio::spawn(async {
        Some(
            fs::read_to_string(format!("/sys/class/power_supply/{}/capacity", "BAT0"))
                .await
                .ok()?
                .parse::<i32>()
                .ok()?,
        )
    });

    let stat = fs::read_to_string(format!("/sys/class/power_supply/{}/status", "BAT0"))
        .await
        .ok()?;

    let icon = if stat == "Discharging" { "" } else { "" };

    let perc = perc.await.unwrap()?;

    Some(
        Block::new(icon, format!("{} %", perc))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}

#[cfg(feature = "bluetooth-battery")]
use dbus::nonblock::{stdintf::org_freedesktop_dbus::Properties, Proxy};
#[cfg(feature = "bluetooth-battery")]
use dbus::Path;
#[cfg(feature = "bluetooth-battery")]
use dbus_tokio::connection;
#[cfg(feature = "bluetooth-battery")]
use std::time::Duration;

/// Build a headset battery component.
/// This functionality depends on UPower DBus daemon
///
/// Return None if no battery device name contains "headset" keyword,
/// or no percentage property is found.
#[cfg(feature = "bluetooth-battery")]
pub async fn headset_battery() -> Option<Block> {
    let (resource, conn) = connection::new_system_sync().ok()?;

    // hold the connection
    let _hold_conn = tokio::spawn(async {
        resource.await;
    });

    let proxy = Proxy::new(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        Duration::from_millis(2000),
        conn.clone(),
    );

    let (devices,): (Vec<Path>,) = proxy
        .method_call("org.freedesktop.UPower", "EnumerateDevices", ())
        .await
        .ok()?;

    let mut device = Path::default();
    for dev in devices {
        if dev.contains("headset") {
            device = dev;
            break;
        }
    }

    if device.is_empty() {
        return None;
    }

    let proxy = Proxy::new(
        "org.freedesktop.UPower",
        device,
        Duration::from_millis(2000),
        conn.clone(),
    );

    let percentage: f64 = proxy
        .get("org.freedesktop.UPower.Device", "Percentage")
        .await
        .ok()?;

    Some(
        Block::new("", format!("{:.0}%", percentage))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}
