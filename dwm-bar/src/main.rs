use std::process::*;

macro_rules! cmd {
    ($c:expr, $($a:expr),*) => {
        {
            let mut args = Vec::new();
            args.push($c);
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
                Ok(s) => s,
                Err(e) => panic!("Unreadable output from command {} {:?}: {}", $c, &args, e),
            }
        }
    }
}

fn main() {
    let date_output = cmd!("date", "'+%B/%d %I:%M %p'");
    println!("{}", date_output);

    let headset_bat_output = cmd!("upower", "-e", "|", "grep", "headset");
    println!("{}", headset_bat_output);
}
