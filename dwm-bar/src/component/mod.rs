mod color;
mod widget;

mod song;
mod datetime;
mod volume;
mod battery;
mod cpu;

// re-export
pub use song::song_info;
pub use datetime::date_and_time;
pub use volume::sound_volume;
#[cfg(feature = "bluetooth-battery")]
pub use battery::headset_battery;
pub use battery::battery;
pub use cpu::avg_load;
