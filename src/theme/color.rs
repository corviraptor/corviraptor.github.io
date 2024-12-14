use palette::blend::Blend;
use palette::rgb::*;
use palette::Hsl;
use palette::Lighten;
use std::format;
use strum_macros::EnumDiscriminants;
use strum_macros::*;

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
    pub fn compare(&self, other: &ColorDomain) -> bool {
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

    pub fn format(&self) -> String {
        Self::style(format!("--{}", &self.domain.get_string()), self.value)
    }

    pub fn shade_and_format(&self, shading: ColorShading) -> String {
        let name = self.domain.get_string() + &shading.to_string();
        Self::style(format!("--{}", &name), shading.on(self.value))
    }

    pub fn style(identifier: String, rgb: Srgba<f32>) -> String {
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
