use palette::rgb::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::*;

use super::{ColorDomain, MainColor, SiteColor};

#[derive(Clone, PartialEq, EnumString, Display, Serialize, Deserialize)]
pub enum ColorTheme {
    #[strum(serialize = "cherry")]
    Cherry,
    #[strum(serialize = "steel")]
    Steel,
}

impl ColorTheme {
    pub fn default_colors(&self) -> Vec<SiteColor> {
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
            MainColor::Disabled => Srgba::new(100, 120, 115, 255),
        };

        SiteColor::new(ColorDomain::Main(color_type), color.into_format())
    }
}
