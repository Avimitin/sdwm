macro_rules! cmd {
    ($c:expr, $($a:expr),*) => {
        {
            let mut args = vec![$c];
            $(
                args.push($a);
            )*
            let raw = Command::new("sh")
                .arg("-c")
                .arg(args.join(" "))
                .output()
                .expect(format!("Fail to execute {} command", $c).as_str());
            let stdout = String::from_utf8(raw.stdout);
            match stdout {
                Ok(s) => s.trim().to_owned(),
                Err(e) => panic!("Unreadable output from command {} {:?}: {}", $c, &args, e),
            }
        }
    };
    ($c:expr) => {
        {
            let raw = Command::new("sh")
                .arg("-c")
                .arg($c)
                .output()
                .expect(format!("Fail to execute {} command", $c).as_str());
            let stdout = String::from_utf8(raw.stdout);
            match stdout {
                Ok(s) => s.trim().to_owned(),
                Err(e) => panic!("Unreadable output from command {}: {}", $c, e),
            }
        }
    }
}

mod color;
mod component;

mod song;
mod datetime;
mod volume;
mod battery;
mod cpu;

// re-export
pub use song::song_info;
pub use datetime::date_and_time;
pub use volume::sound_volume;
pub use battery::{battery, headset_battery};
pub use cpu::avg_load;
