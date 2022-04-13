use std::fs;
use super::component::Component;

pub fn avg_load() -> Option<Component> {
    let status = fs::read_to_string("/proc/stat").ok()?;
    let mut cpustat = Vec::new();
    for line in status.lines() {
        if line.starts_with("cpu") {
            cpustat = line.split(' ').skip(2).collect::<Vec<&str>>();
            break;
        }
    }

    if cpustat.len() < 8 {
        return None;
    }

    // get the cpu idle time
    let idle = cpustat.remove(3).parse::<f32>().ok()?;
    let mut active = 0.0;
    for time in cpustat {
        active += time.parse::<f32>().ok()?;
    }

    let avg = active / (active + idle);

    Some(
        Component::new("﬙", format!("{:.2} %", avg * 100.0))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}
