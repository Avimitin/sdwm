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
    pub fn date_and_time() -> Component {
        // TODO: use rust native date time
        let date_output = cmd!("date", "'+%B/%d %I:%M %p'");
        Component::new("", &date_output, "#EAEAEA", "")
    }

    /// Create a sound volume component for bar
    pub fn sound_volume() -> Component {
        let output = cmd!("pamixer", "--get-volume");
        Component::new("", format!("{}%", output).as_str(), "#EAEAEA", "")
    }
}

/// Reset the color the SchemeNorm
static NORMAL_COLOR: &str = "^d^";

use std::io::{self, Write};

fn main() {
    let bar = vec![
        component::sound_volume(),
        component::date_and_time(),
    ];

    for comp in bar {
        // TODO: make separater more flexible to DIY
        print!(" | ");
        print!("{}", comp);
        print!("{}", NORMAL_COLOR);
    }
    io::stdout().flush().unwrap()
}
