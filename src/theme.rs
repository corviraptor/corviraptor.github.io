use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Theme {
    pub styles: Vec<StyleEntry>,
    pub crt_active: bool,
}

#[derive(Clone, PartialEq)]
pub struct StyleEntry(StyleType, Style);

#[derive(Clone, PartialEq)]
pub enum StyleType {
    Main,
    MainDark,
    MainDarker,
    MainHighlighted,
    MainBright,
    Accent,
    AccentHighlighted,
    ButtonDisabled,
    Background,
    TextColor,

    Font,
}

impl StyleType {
    fn default_style(&self) -> Style {
        match self {
            StyleType::Main => Style::new("--main", "#b91647"),
            StyleType::MainDark => Style::new("--main-dark", "#820839"),
            StyleType::MainDarker => Style::new("--main-darker", "#530e29"),
            StyleType::MainHighlighted => Style::new("--main-highlighted", "#d62a55"),
            StyleType::MainBright => Style::new("--main-bright", "#fa4e79"),
            StyleType::Accent => Style::new("--accent", "#d9e4e0"),
            StyleType::AccentHighlighted => Style::new("--accent-highlighted", "#ffffff"),
            StyleType::ButtonDisabled => Style::new("--button-disabled", "#81968e"),
            StyleType::Background => Style::new("--background", "#1d1b1b"),
            StyleType::TextColor => Style::new("--text-color", "#000000"),
            StyleType::Font => Style::new("font-family", "'Iosevka Corax Web', monospace"),
        }
    }

    fn default_entry(&self) -> StyleEntry {
        StyleEntry(self.clone(), self.default_style())
    }
}

#[derive(Clone, PartialEq)]
pub struct Style {
    pub name: String,
    pub value: String,
}

impl Style {
    fn new(name: &str, value: &str) -> Self {
        Style {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    fn with(&self, value: String) -> Style {
        Style {
            value,
            ..self.clone()
        }
    }
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            styles: Self::default_styles(),
            crt_active: true,
        }
    }

    pub fn default_styles() -> Vec<StyleEntry> {
        vec![
            StyleType::Main.default_entry(),
            StyleType::MainDark.default_entry(),
            StyleType::MainDarker.default_entry(),
            StyleType::MainHighlighted.default_entry(),
            StyleType::MainBright.default_entry(),
            StyleType::Accent.default_entry(),
            StyleType::AccentHighlighted.default_entry(),
            StyleType::ButtonDisabled.default_entry(),
            StyleType::Background.default_entry(),
            StyleType::TextColor.default_entry(),
            StyleType::Font.default_entry(),
        ]
    }

    pub fn get_theme_string(&self) -> String {
        let mut theme_string = "".to_string();
        for style_entry in &self.styles {
            theme_string += &format!("{}: {};", style_entry.1.name, style_entry.1.value);
        }
        theme_string
    }

    pub fn get_crt_overlay(&self) -> Option<Classes> {
        if self.crt_active {
            Some(classes!("crt", "ca-text"))
        } else {
            None
        }
    }

    pub fn get_style(&self, style_type: StyleType) -> Option<Style> {
        let entry = self.styles.iter().find(|x| x.0 == style_type);

        entry.map(|x| x.clone().1)
    }

    pub fn with_style(&self, style_type: StyleType, value: String) -> Theme {
        let mut styles = self.clone().styles;
        if let Some(x) = styles.iter().position(|x| x.0 == style_type) {
            let entry = StyleEntry(style_type, styles[x].1.with(value));
            styles.remove(x);
            styles.insert(x, entry);
        }

        Theme {
            styles,
            ..self.clone()
        }
    }
}
