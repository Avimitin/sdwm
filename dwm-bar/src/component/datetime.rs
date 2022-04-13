/// Create a date component for bar
use super::component::Component;
use chrono::prelude::Local;

pub fn date_and_time() -> Option<Component> {
    let now = Local::now();
    Some(
        Component::new("", now.format("%B/%d %I:%M %p").to_string())
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}

