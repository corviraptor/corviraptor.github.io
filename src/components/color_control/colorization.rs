use palette::Srgb;

#[derive(Debug, Clone, PartialEq)]
pub struct Colorization(pub Vec<Srgb<f32>>);

impl Default for Colorization {
    fn default() -> Self {
        Self(vec![palette::named::WHITE.into_format()])
    }
}

pub fn get_gradient_style(colors: Vec<Srgb>) -> String {
    format!(
        "background: linear-gradient(to right, {}); -webkit-background-clip: text; background-clip: text; color: transparent;",
        colors
            .iter()
            .map(get_css_rgb)
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub fn get_css_rgb(color: &Srgb) -> String {
    let components: (u8, u8, u8) = color.into_format::<u8>().into_components();
    format!(
        "rgb({}, {}, {}, 1)",
        components.0, components.1, components.2
    )
}

#[cfg(test)]
mod tests {
    use palette::named::{BLUE, GREEN, RED};

    use super::*;
    #[test]
    fn test_gradient_style() {
        let output = get_gradient_style(vec![
            BLUE.into_format(),
            RED.into_format(),
            GREEN.into_format(),
        ]);
        println!("{:#?}", output);
    }
}
