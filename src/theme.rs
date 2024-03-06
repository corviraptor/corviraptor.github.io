use palette::blend::Blend;
use palette::rgb::*;
use palette::Hsl;
use palette::Lighten;
use std::format;
use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, EnumString, Display, EnumIter)]
pub enum MainColor {
    #[strum(serialize = "main")]
    Main,
    #[strum(serialize = "accent")]
    Accent,
    #[strum(serialize = "highlight")]
    Highlight,
    #[strum(serialize = "disabled")]
    Disabled,
}

impl MainColor {
    fn compare(&self, other: &ColorDomain) -> bool {
        if let ColorDomain::Main(x) = other {
            self == x
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq, EnumString, Display, EnumIter)]
pub enum ScreenColor {
    #[strum(serialize = "body")]
    Body,
    #[strum(serialize = "text-color")]
    Main,
    #[strum(serialize = "text-highlight")]
    Highlight,
    #[strum(serialize = "screen-border")]
    Border,
}

impl ScreenColor {
    fn compare(&self, other: &ColorDomain) -> bool {
        if let ColorDomain::Text(x) = other {
            self == x
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq, EnumDiscriminants)]
pub enum ColorDomain {
    Main(MainColor),
    Text(ScreenColor),
}

impl ColorDomain {
    pub fn get_string(&self) -> String {
        match self {
            ColorDomain::Main(x) => x.to_string(),
            ColorDomain::Text(x) => x.to_string(),
        }
    }

    // this feels stupid
    fn compare(&self, other: &ColorDomain) -> bool {
        match self {
            ColorDomain::Main(x) => x.compare(other),
            ColorDomain::Text(x) => x.compare(other),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SiteColor {
    pub domain: ColorDomain,
    pub value: Srgba<f32>,
}

impl SiteColor {
    pub fn new(domain: ColorDomain, value: Srgba<f32>) -> Self {
        SiteColor { domain, value }
    }

    fn format(&self) -> String {
        Self::style(format!("--{}", &self.domain.get_string()), self.value)
    }

    fn shade_and_format(&self, shading: ColorShading) -> String {
        let name = self.domain.get_string() + &shading.to_string();
        Self::style(format!("--{}", &name), shading.on(self.value))
    }

    fn style(identifier: String, rgb: Srgba<f32>) -> String {
        let components: (u8, u8, u8, u8) = rgb.into_format().into_components();
        let rgb_string = format!(
            "rgb({}, {}, {}, {})",
            components.0, components.1, components.2, components.3
        );
        format!("{}: {};", identifier, rgb_string)
    }
}

#[derive(Clone, PartialEq, Display)]
pub enum ColorShading {
    #[strum(to_string = "-dark")]
    Dark,
    #[strum(to_string = "-darker")]
    Darker,
    #[strum(to_string = "-light")]
    Light,
    #[strum(to_string = "-lighter")]
    Lighter,
}

impl ColorShading {
    fn on(&self, color: Srgba<f32>) -> Srgba<f32> {
        let components: (f32, f32, f32, _) = color.into_components();
        let lightness_factor = Hsl::new_srgb(components.0, components.1, components.2).lightness;
        let l_color = color.into_linear();
        match self {
            ColorShading::Dark => l_color.multiply(Srgba::new(0.4, 0.0, 0.2, 0.5).into_linear()),
            ColorShading::Darker => l_color.multiply(Srgba::new(0.4, 0.0, 0.2, 0.8).into_linear()),
            ColorShading::Light => l_color.lighten_fixed(0.2 * lightness_factor),
            ColorShading::Lighter => l_color.lighten_fixed(0.4 * lightness_factor),
        }
        .into()
    }
}

#[derive(Clone, PartialEq, EnumString, Display)]
pub enum SiteFont {
    #[strum(serialize = "Iosevka Corax")]
    IosevkaCorax,
    #[strum(serialize = "Iosevka")]
    Iosevka,
    #[strum(serialize = "Source Serif 4")]
    SourceSerif4,
    #[strum(serialize = "Atkinson Hyperlegible")]
    AtkinsonHyperlegible,
    #[strum(serialize = "System Default")]
    Default,
}

impl SiteFont {
    fn style(&self) -> String {
        let suffix = match self {
            SiteFont::IosevkaCorax => "'Iosevka Corax', 'Symbols Nerd Font', monospace",
            SiteFont::Iosevka => "'Iosevka Web', 'Symbols Nerd Font', monospace",
            SiteFont::SourceSerif4 => "'Source Serif 4', 'Symbols Nerd Font', serif",
            SiteFont::AtkinsonHyperlegible => "'Atkinson Hyperlegible', 'Symbols Nerd Font', sans-serif",
            SiteFont::Default => "system-ui, 'Symbols Nerd Font'",
        }
        .to_string();

        format!("font-family: {};", suffix)
    }
}

#[derive(Clone, PartialEq, EnumString, Display)]
pub enum ColorTheme {
    #[strum(serialize = "cherry")]
    Cherry,
    #[strum(serialize = "steel")]
    Steel,
}

impl ColorTheme {
    fn default_colors(&self) -> Vec<SiteColor> {
        match self {
            ColorTheme::Cherry => self.cherry(),
            ColorTheme::Steel => self.terminal(),
        }
    }

    pub fn default_color(&self, color: MainColor) -> SiteColor {
        match self {
            ColorTheme::Cherry => self.default_cherry(color),
            ColorTheme::Steel => self.default_steel(color),
        }
    }

    fn cherry(&self) -> Vec<SiteColor> {
        let mut color_buf: Vec<SiteColor> = vec![];
        for color in MainColor::iter() {
            color_buf.push(self.default_cherry(color));
        }
        color_buf
    }

    fn terminal(&self) -> Vec<SiteColor> {
        let mut color_buf: Vec<SiteColor> = vec![];
        for color in MainColor::iter() {
            color_buf.push(self.default_steel(color));
        }
        color_buf
    }

    fn default_cherry(&self, color_type: MainColor) -> SiteColor {
        let color: Srgba<u8> = match color_type {
            MainColor::Main => Srgba::new(185, 22, 71, 255),
            MainColor::Accent => Srgba::new(217, 228, 224, 255),
            MainColor::Highlight => Srgba::new(255, 255, 255, 255),
            MainColor::Disabled => Srgba::new(129, 150, 142, 255),
        };

        SiteColor::new(ColorDomain::Main(color_type), color.into_format())
    }

    fn default_steel(&self, color_type: MainColor) -> SiteColor {
        let color: Srgba<u8> = match color_type {
            MainColor::Main => Srgba::new(30, 30, 30, 255),
            MainColor::Accent => Srgba::new(185, 22, 71, 255),
            MainColor::Highlight => Srgba::new(255, 255, 255, 255),
            MainColor::Disabled => Srgba::new(3, 3, 3, 255),
        };

        SiteColor::new(ColorDomain::Main(color_type), color.into_format())
    }
}

#[derive(Clone, PartialEq, EnumString, Display)]
pub enum TextTheme {
    #[strum(serialize = "terminal")]
    Terminal,
    #[strum(serialize = "calculator")]
    Calculator,
}

impl TextTheme {
    fn default_colors(&self) -> Vec<SiteColor> {
        match self {
            TextTheme::Terminal => self.terminal(),
            TextTheme::Calculator => self.calculator(),
        }
    }

    pub fn default_color(&self, color: ScreenColor) -> SiteColor {
        match self {
            TextTheme::Terminal => self.default_terminal(color),
            TextTheme::Calculator => self.default_calculator(color),
        }
    }

    fn terminal(&self) -> Vec<SiteColor> {
        let mut color_buf: Vec<SiteColor> = vec![];
        for color in ScreenColor::iter() {
            color_buf.push(self.default_terminal(color));
        }
        color_buf
    }

    fn calculator(&self) -> Vec<SiteColor> {
        let mut color_buf: Vec<SiteColor> = vec![];
        for color in ScreenColor::iter() {
            color_buf.push(self.default_calculator(color));
        }
        color_buf
    }

    fn default_terminal(&self, color_type: ScreenColor) -> SiteColor {
        let color: Srgba<u8> = match color_type {
            ScreenColor::Body => Srgba::new(0, 0, 0, 255),
            ScreenColor::Main => Srgba::new(217, 228, 224, 255),
            ScreenColor::Highlight => Srgba::new(255, 0, 100, 255),
            ScreenColor::Border => Srgba::new(217, 228, 224, 255),
        };

        SiteColor::new(ColorDomain::Text(color_type), color.into_format())
    }

    fn default_calculator(&self, color_type: ScreenColor) -> SiteColor {
        let color: Srgba<u8> = match color_type {
            ScreenColor::Body => Srgba::new(217, 228, 224, 255),
            ScreenColor::Main => Srgba::new(0, 0, 0, 255),
            ScreenColor::Highlight => Srgba::new(255, 0, 100, 255),
            ScreenColor::Border => Srgba::new(255, 24, 104, 255),
        };

        SiteColor::new(ColorDomain::Text(color_type), color.into_format())
    }
}

#[derive(Clone, PartialEq)]
pub struct Theme {
    pub color_theme: ColorTheme,
    pub text_theme: TextTheme,
    pub custom_colors: Vec<SiteColor>,
    pub font: SiteFont,
    pub crt_active: bool,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            color_theme: ColorTheme::Cherry,
            text_theme: TextTheme::Terminal,
            crt_active: true,
            custom_colors: Vec::<SiteColor>::new(),
            font: SiteFont::IosevkaCorax,
        }
    }

    pub fn get_theme_string(&self) -> String {
        let mut theme_string = String::new();

        let color_buf: Vec<SiteColor> = self
            .color_theme
            .default_colors()
            .into_iter()
            .chain(self.text_theme.default_colors())
            .chain(self.custom_colors.clone())
            .collect();

        for color in color_buf {
            theme_string += &color.format();
            theme_string += &color.shade_and_format(ColorShading::Dark);
            theme_string += &color.shade_and_format(ColorShading::Darker);
            theme_string += &color.shade_and_format(ColorShading::Light);
            theme_string += &color.shade_and_format(ColorShading::Lighter);
        }

        theme_string += &self.font.style();
        theme_string
    }

    pub fn get_crt_overlay(&self) -> Option<Classes> {
        if self.crt_active {
            Some(classes!("crt", "ca-text"))
        } else {
            None
        }
    }

    pub fn with_color(&self, color: &SiteColor) -> Theme {
        let mut colors_buf: Vec<SiteColor> = self.custom_colors.clone();

        colors_buf.retain(|e| !e.domain.compare(&color.domain));

        colors_buf.push(color.clone());

        Theme {
            custom_colors: colors_buf,
            ..self.clone()
        }
    }

    pub fn without_color(&self, color: &ColorDomain) -> Theme {
        let mut colors_buf: Vec<SiteColor> = self.custom_colors.clone();

        colors_buf.retain(|e| !e.domain.compare(color));

        Theme {
            custom_colors: colors_buf,
            ..self.clone()
        }
    }

    pub fn with_font(&self, font: SiteFont) -> Theme {
        Theme {
            font,
            ..self.clone()
        }
    }

    pub fn with_color_theme(&self, color_theme: ColorTheme) -> Theme {
        Theme {
            color_theme,
            ..self.clone()
        }
    }

    pub fn with_text_theme(&self, text_theme: TextTheme) -> Theme {
        Theme {
            text_theme,
            ..self.clone()
        }
    }
}
