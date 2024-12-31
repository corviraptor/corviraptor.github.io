use serde::{Deserialize, Serialize};
use std::format;
use strum_macros::*;

#[derive(Clone, PartialEq, EnumString, Display, Serialize, Deserialize)]
pub enum SiteFont {
    #[strum(serialize = "Iosevka Slab Web")]
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
    pub fn style(&self) -> String {
        let suffix = match self {
            SiteFont::IosevkaCorax => "'Iosevka Slab Web', 'Symbols Nerd Font', monospace",
            SiteFont::Iosevka => "'Iosevka Web', 'Symbols Nerd Font', monospace",
            SiteFont::SourceSerif4 => "'Source Serif 4', 'Symbols Nerd Font', serif",
            SiteFont::AtkinsonHyperlegible => {
                "'Atkinson Hyperlegible', 'Symbols Nerd Font', sans-serif"
            }
            SiteFont::Default => "system-ui, 'Symbols Nerd Font'",
        }
        .to_string();

        format!("font-family: {};", suffix)
    }
}
