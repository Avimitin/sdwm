mod component;

/// Reset the color the SchemeNorm
static NORMAL_COLOR: &str = "^d^";
static DIVIDER: &str = "     |     ";

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn run() {
    loop {
        let bar = vec![
            component::song_info(),
            component::sound_volume(),
            component::headset_battery(),
            component::battery(),
            component::avg_load(),
            component::date_and_time(),
        ];

        let mut begining = true;
        let mut barline = String::new();
        for component in bar.iter().flatten() {
            if begining {
                begining = false;
            } else {
                barline.push_str(DIVIDER);
            }
            barline.push_str(&format!("{}", component));
            barline.push_str(NORMAL_COLOR);
        }

        if let Ok(mut child) = Command::new("xsetroot").arg("-name").arg(barline).spawn() {
            child.wait().expect("fail to end the xsetroot command");
        } else {
            eprintln!("Fail to execute xsetroot")
        }

        sleep(Duration::from_secs(10));
    }
}

fn main() {
    run()
}
