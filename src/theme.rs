pub mod color;
pub mod color_theme;
pub mod font;

use palette::rgb::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::*;
use yew::prelude::*;

use color::*;
use color_theme::*;
use font::*;

use crate::storage;

#[derive(Clone, PartialEq, EnumString, Display, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub color_theme: ColorTheme,
    pub text_theme: TextTheme,
    pub custom_colors: Vec<SiteColor>,
    pub font: SiteFont,
    pub crt_active: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn from_storage() -> Self {
        storage::get("theme").unwrap_or_default()
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

    pub fn with_crt_setting(&self, crt_active: bool) -> Theme {
        Theme {
            crt_active,
            ..self.clone()
        }
    }

    pub fn store_to_site(&self, state_handle: &UseStateHandle<Theme>) {
        state_handle.set(self.clone());
        let _ = storage::set("theme", self);
    }
}
