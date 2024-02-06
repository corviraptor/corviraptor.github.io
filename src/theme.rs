use palette::blend::Blend;
use palette::rgb::*;
use palette::Hsl;
use palette::Lighten;
use phf::phf_map;
use std::fmt;
use std::mem;
use yew::prelude::*;

pub static FONTS: phf::Map<&'static str, SiteFont> = phf_map! {
    "Iosevka Corax" => SiteFont::IosevkaCorax,
    "Iosevka" => SiteFont::Iosevka,
    "Source Serif 4" => SiteFont::SourceSerif4,
    "Atkinson Hyperlegible" => SiteFont::AtkinsonHyperlegible,
    "Monospace" => SiteFont::Default(FontVariant::Monospace),
    "Serif" => SiteFont::Default(FontVariant::Serif),
    "Sans Serif" => SiteFont::Default(FontVariant::SansSerif),
};

pub static COLOR_THEMES: phf::Map<&'static str, ColorTheme> = phf_map! {
    "Light" => ColorTheme::Light,
    "Dark" => ColorTheme::Dark,
};

pub static COLORS: phf::Map<&'static str, ColorType> = phf_map! {
    "Main" => ColorType::Main,
    "Accent" => ColorType::Accent,
    "Highlight" => ColorType::Highlight,
    "Body" => ColorType::Body,
    "Disabled" => ColorType::Disabled,
    "Text Color" => ColorType::TextColor,
    "Text Highlight" => ColorType::TextHighlight,
};

#[derive(Clone, PartialEq)]
pub enum ColorType {
    Main,
    Accent,
    Highlight,
    Body,
    Disabled,
    TextColor,
    TextHighlight,
}

impl ColorType {
    pub fn get_id(&self) -> String {
        COLORS
            .entries
            .iter()
            .find(|e| e.1 == *self)
            .map(|e| e.0)
            .unwrap()
            .to_owned()
    }
}

impl fmt::Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {
            f,
            "{}",
            match self {
                ColorType::Main => "--main",
                ColorType::Accent => "--accent",
                ColorType::Highlight => "--highlight",
                ColorType::Body => "--body",
                ColorType::Disabled => "--disabled",
                ColorType::TextColor => "--text-color",
                ColorType::TextHighlight => "--text-highlight",
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ColorShading {
    Dark,
    Darker,
    Light,
    Lighter,
}

impl ColorShading {
    fn on(&self, color: Srgba<f32>) -> Srgba<f32> {
        let components: (f32, f32, f32, _) = color.into_components();
        let lightness_factor = Hsl::new_srgb(components.0, components.1, components.2).lightness;
        match self {
            ColorShading::Dark => color
                .into_linear()
                .multiply(Srgba::new(0.4, 0.0, 0.2, 0.5).into_linear())
                .into(),
            ColorShading::Darker => color
                .into_linear()
                .multiply(Srgba::new(0.4, 0.0, 0.2, 0.8).into_linear())
                .into(),
            ColorShading::Light => color
                .into_linear()
                .lighten_fixed(0.2 * lightness_factor)
                .into(),
            ColorShading::Lighter => color
                .into_linear()
                .lighten_fixed(0.4 * lightness_factor)
                .into(),
        }
    }
}

impl fmt::Display for ColorShading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {
            f,
            "{}",
            match self {
                ColorShading::Dark => "-dark",
                ColorShading::Darker => "-darker",
                ColorShading::Light => "-light",
                ColorShading::Lighter => "-lighter",
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SiteColor(pub ColorType, pub Srgba<f32>);

impl SiteColor {
    fn format(&self) -> String {
        Self::style(self.0.to_string(), self.1)
    }

    fn shade_and_format(&self, shading: ColorShading) -> String {
        let name = self.0.to_string() + &shading.to_string();
        Self::style(name, shading.on(self.1))
    }

    fn style(identifier: String, rgb: Srgba<f32>) -> String {
        let components: (u8, u8, u8, u8) = rgb.into_format().into_components();
        let rgb_string = format!("rgb({}, {}, {})", components.0, components.1, components.2);
        format!("{}: {};", identifier, rgb_string)
    }
}

#[derive(Clone, PartialEq)]
pub enum FontVariant {
    Monospace,
    Serif,
    SansSerif,
}

impl FontVariant {
    fn style(&self) -> String {
        match self {
            FontVariant::Monospace => "monospace".to_string(),
            FontVariant::Serif => "serif".to_string(),
            FontVariant::SansSerif => "sans-serif".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum SiteFont {
    IosevkaCorax,
    Iosevka,
    SourceSerif4,
    AtkinsonHyperlegible,
    Default(FontVariant),
}

impl SiteFont {
    pub fn from_str(variant: &str) -> Self {
        FONTS.get(variant).unwrap().clone()
    }

    fn style(&self) -> String {
        let suffix = match self {
            SiteFont::IosevkaCorax => "'Iosevka Corax Web', 'Iosevka Web', monospace".to_string(),
            SiteFont::Iosevka => "'Iosevka Web', monospace".to_string(),
            SiteFont::SourceSerif4 => "'Source Serif 4', serif".to_string(),
            SiteFont::AtkinsonHyperlegible => "'Atkinson Hyperlegible, sans-serif'".to_string(),
            SiteFont::Default(x) => x.style(),
        };
        format!("font-family: {};", suffix)
    }
}

impl fmt::Display for SiteFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {
            f,
            "{}",
            FONTS.entries().find(|e| e.1 == self).unwrap().0
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ColorTheme {
    Light,
    Dark,
}

impl ColorTheme {
    pub fn from_str(variant: &str) -> Self {
        COLOR_THEMES.get(variant).unwrap().clone()
    }

    fn default_colors(&self) -> Vec<SiteColor> {
        match self {
            ColorTheme::Light => self.light_theme(),
            ColorTheme::Dark => self.dark_theme(),
        }
    }

    pub fn default_color(&self, color: ColorType) -> SiteColor {
        match self {
            ColorTheme::Light => self.default_light(color),
            ColorTheme::Dark => self.default_dark(color),
        }
    }

    fn light_theme(&self) -> Vec<SiteColor> {
        vec![
            self.default_light(ColorType::Main),
            self.default_light(ColorType::Accent),
            self.default_light(ColorType::Highlight),
            self.default_light(ColorType::Body),
            self.default_light(ColorType::Disabled),
            self.default_light(ColorType::TextColor),
            self.default_light(ColorType::TextHighlight),
        ]
    }

    fn dark_theme(&self) -> Vec<SiteColor> {
        vec![
            self.default_dark(ColorType::Main),
            self.default_dark(ColorType::Accent),
            self.default_dark(ColorType::Highlight),
            self.default_dark(ColorType::Body),
            self.default_dark(ColorType::Disabled),
            self.default_dark(ColorType::TextColor),
            self.default_dark(ColorType::TextHighlight),
        ]
    }

    fn default_light(&self, color_type: ColorType) -> SiteColor {
        let color: Srgba<u8> = match color_type {
            ColorType::Main => Srgba::new(185, 22, 71, 255),
            ColorType::Accent => Srgba::new(217, 228, 224, 255),
            ColorType::Highlight => Srgba::new(255, 255, 255, 255),
            ColorType::Body => Srgba::new(0, 0, 0, 255),
            ColorType::Disabled => Srgba::new(129, 150, 142, 255),
            ColorType::TextColor => Srgba::new(217, 228, 224, 255),
            ColorType::TextHighlight => Srgba::new(255, 24, 104, 255),
        };

        SiteColor(color_type, color.into_format())
    }

    fn default_dark(&self, color_type: ColorType) -> SiteColor {
        let color: Srgba<u8> = match color_type {
            ColorType::Main => Srgba::new(30, 30, 30, 255),
            ColorType::Accent => Srgba::new(185, 22, 71, 255),
            ColorType::Highlight => Srgba::new(255, 255, 255, 255),
            ColorType::Body => Srgba::new(0, 0, 0, 255),
            ColorType::Disabled => Srgba::new(3, 3, 3, 255),
            ColorType::TextColor => Srgba::new(217, 228, 224, 255),
            ColorType::TextHighlight => Srgba::new(255, 24, 104, 255),
        };

        SiteColor(color_type, color.into_format())
    }
}

impl fmt::Display for ColorTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {
            f,
            "{}",
            COLOR_THEMES.entries().find(|e| e.1 == self).unwrap().0
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Theme {
    pub color_theme: ColorTheme,
    pub custom_colors: Vec<SiteColor>,
    pub font: SiteFont,
    pub crt_active: bool,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            color_theme: ColorTheme::Light,
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
        colors_buf.retain(|e| mem::discriminant(&e.0) != mem::discriminant(&color.0));
        colors_buf.push(color.clone());
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
}
