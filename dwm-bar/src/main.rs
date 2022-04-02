mod component {
    use std::process::*;

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

    #[derive(Debug, PartialEq)]
    struct Color {
        fg: Option<String>,
        bg: Option<String>,
    }

    impl Color {
        /// Create a new color set. The first parameter `fg` means foreground, and the second
        /// parameter `bg` means background. You should give a valid hex color code in format
        /// like `#FFFFFF`. If you pass empty string, this means you are going to use the
        /// default colors.
        fn new<T: Into<String>>(fg: T, bg: T) -> Self {
            let fg = fg.into();
            let bg = bg.into();

            let fg = if fg.is_empty() {
                None
            } else {
                Some(format!("^c{}^", fg))
            };

            let bg = if bg.is_empty() {
                None
            } else {
                Some(format!("^b{}^", bg))
            };

            Self { fg, bg }
        }
    }

    #[test]
    fn test_color_new() {
        assert_eq!(Color::new("", ""), Color { fg: None, bg: None });
        assert_eq!(
            Color::new("#000000", ""),
            Color {
                fg: Some("^c#000000^".to_string()),
                bg: None
            }
        );
        assert_eq!(
            Color::new("", "#FFFFFF"),
            Color {
                fg: None,
                bg: Some("^b#FFFFFF^".to_string())
            }
        );
        assert_eq!(
            Color::new("#000000", "#FFFFFF"),
            Color {
                fg: Some("^c#000000^".to_string()),
                bg: Some("^b#FFFFFF^".to_string())
            }
        );
    }

    #[derive(Debug)]
    pub struct Component {
        color: Color,
        text: String,
        icon: String,
    }

    impl Component {
        /// Create a new component with icon, text, and foreground, backgroun colors.
        ///
        /// T: Into<String>
        pub fn new<T: Into<String>>(icon: T, text: T, fg: T, bg: T) -> Self {
            Self {
                icon: icon.into(),
                text: text.into(),
                color: Color::new(fg, bg),
            }
        }
    }

    impl std::fmt::Display for Component {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut fg = String::new();
            let mut bg = String::new();
            if let Some(s) = &self.color.fg {
                fg = s.clone();
            }
            if let Some(s) = &self.color.bg {
                bg = s.clone();
            }
            // [icon] [text]
            write!(f, "{} {}{}{}", self.icon, fg, bg, self.text)
        }
    }

    /// Create a date component for bar
    pub fn date_and_time() -> Option<Component> {
        // TODO: use rust native date time
        let date_output = cmd!("date", "'+%B/%d %I:%M %p'");
        Some(Component::new("", &date_output, "#EAEAEA", ""))
    }

    /// Create a sound volume component for bar
    pub fn sound_volume() -> Option<Component> {
        // TODO: Can we RIIR this? Or do Rust have PulseAudio library? Figure it out!
        let output = cmd!("pamixer", "--get-volume");
        Some(Component::new(
            "",
            format!("{}%", output).as_str(),
            "#EAEAEA",
            "",
        ))
    }

    pub fn song_info() -> Option<Component> {
        let text_limit = 40;

        let artist = cmd!("playerctl", "metadata", "artist");
        let song = cmd!("playerctl", "metadata", "title");

        // No music player is open
        if artist.starts_with("No player could handle this command") {
            return None;
        }

        let output = format!(
            "{} - {}",
            if !artist.is_empty() {
                artist
            } else {
                "Anonymous".to_string()
            },
            song,
        );

        Some(Component::new(
            "",
            if output.len() > text_limit {
                &output[0..text_limit]
            } else {
                &output
            },
            "",
            "#0c0c0c",
        ))
    }

    pub fn battery() -> Option<Component> {
        let output = cmd!("acpi");
        let output: Vec<&str> = output.split(": ").collect();
        if output.len() < 2 {
            return None;
        }

        let status = output[1];
        let status: Vec<&str> = status.split(", ").collect();
        if status[0] == "Discharging" {
            Some(Component::new("", status[1], "#EAEAEA", ""))
        } else {
            Some(Component::new("", status[1], "#EAEAEA", ""))
        }
    }
}

/// Reset the color the SchemeNorm
static NORMAL_COLOR: &str = "^d^";

use std::io::{self, Write};

fn main() {
    let bar = vec![
        component::song_info(),
        component::sound_volume(),
        component::battery(),
        component::date_and_time(),
    ];

    let mut begining = true;
    for component in bar.iter().flatten() {
        if begining {
            begining = false;
        } else {
        // TODO: make separater more flexible to DIY
            print!(" | ");
        }
        print!("{}", component);
        print!("{}", NORMAL_COLOR);
    }
    io::stdout().flush().unwrap()
}
