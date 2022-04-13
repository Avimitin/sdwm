use super::component::Component;
use std::process::Command;

/// Create a sound volume component for bar
pub fn sound_volume() -> Option<Component> {
    // TODO: use the libpulse crates to do this shit
    let output = cmd!("pamixer", "--get-volume");
    Some(
        Component::new("", format!("{}%", output))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}

