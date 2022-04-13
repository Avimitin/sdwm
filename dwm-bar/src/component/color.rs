#[derive(Debug, PartialEq, Default)]
pub struct Color {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub icon_fg: Option<String>,
    pub icon_bg: Option<String>,
}

impl Color {
    /// Create a new color set. The first parameter `fg` means foreground, and the second
    /// parameter `bg` means background. You should give a valid hex color code in format
    /// like `#FFFFFF`. If you pass empty string, this means you are going to use the
    /// default colors.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text<T: Into<String>, E: Into<String>>(mut self, fg: T, bg: E) -> Self {
        self.fg = Some(format!("^c{}^", fg.into()));
        self.bg = Some(format!("^b{}^", bg.into()));
        self
    }

    pub fn icon<T: Into<String>, E: Into<String>>(mut self, fg: T, bg: E) -> Self {
        self.icon_fg = Some(format!("^c{}^", fg.into()));
        self.icon_bg = Some(format!("^b{}^", bg.into()));
        self
    }

    pub fn text_fg<T: Into<String>>(mut self, fg: T) -> Self {
        self.fg = Some(format!("^c{}^", fg.into()));
        self
    }

    #[allow(dead_code)]
    pub fn text_bg<T: Into<String>>(mut self, bg: T) -> Self {
        self.bg = Some(format!("^b{}^", bg.into()));
        self
    }

    #[allow(dead_code)]
    pub fn icon_fg<T: Into<String>>(mut self, fg: T) -> Self {
        self.icon_fg = Some(format!("^c{}^", fg.into()));
        self
    }

    #[allow(dead_code)]
    pub fn icon_bg<T: Into<String>>(mut self, bg: T) -> Self {
        self.icon_bg = Some(format!("^b{}^", bg.into()));
        self
    }
}


#[test]
fn test_color_new() {
    assert_eq!(
        Color::new(),
        Color {
            fg: None,
            bg: None,
            icon_fg: None,
            icon_bg: None
        }
    );
    assert_eq!(
        Color::new().text_fg("#000000").icon_fg("#000000"),
        Color {
            fg: Some("^c#000000^".to_string()),
            bg: None,
            icon_fg: Some("^c#000000^".to_string()),
            icon_bg: None,
        }
    );
    assert_eq!(
        Color::new().text_bg("#FFFFFF").icon_bg("#FFFFFF"),
        Color {
            fg: None,
            bg: Some("^b#FFFFFF^".to_string()),
            icon_fg: None,
            icon_bg: Some("^b#FFFFFF^".to_string())
        }
    );
    assert_eq!(
        Color::new().text("#000000", "#FFFFFF"),
        Color {
            fg: Some("^c#000000^".to_string()),
            bg: Some("^b#FFFFFF^".to_string()),
            icon_fg: None,
            icon_bg: None,
        }
    );
    assert_eq!(
        Color::new().icon("#EAEAEA", "#FF00FF"),
        Color {
            fg: Some("^c#000000^".to_string()),
            bg: Some("^b#FFFFFF^".to_string()),
            icon_fg: Some("^c#EAEAEA^".to_string()),
            icon_bg: Some("^b#FF00FF^".to_string()),
        }
    );
}

