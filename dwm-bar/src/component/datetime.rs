/// Create a date component for bar
use super::widget::Block;
use chrono::prelude::Local;

pub fn date_and_time() -> Option<Block> {
    let now = Local::now();
    Some(
        Block::new("", now.format("%B/%d %I:%M %p").to_string())
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}

