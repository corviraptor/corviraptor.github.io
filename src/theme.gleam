import gleam/bool
import gleam/float
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import lustre/attribute
import niji

pub type Theme {
  Theme(
    opaque_background: Bool,
    text_stroke: Bool,
    accent: niji.Rgb,
    text_background: niji.Rgb,
    text_color: niji.Rgb,
  )
}

// TODO: make this not stupid
pub type ThemeColor {
  Accent
  TextBackground
  TextColor
}

pub fn get_default_theme_color(color: ThemeColor) {
  case color {
    Accent -> default_hex_to_rgb("cd125d")
    TextBackground -> default_hex_to_rgb("241C52")
    TextColor -> niji.rgb(1.0, 1.0, 1.0, 1.0)
  }
}

pub fn get_id(color: ThemeColor) {
  case color {
    Accent -> "accent-main"
    TextBackground -> "text-background"
    TextColor -> "text-color"
  }
}

pub fn set_theme_rgb(old_theme: Theme, color_type: ThemeColor, color) {
  case color_type {
    Accent -> Theme(..old_theme, accent: color)
    TextBackground -> Theme(..old_theme, text_background: color)
    TextColor -> Theme(..old_theme, text_color: color)
  }
}

pub fn is_shaded_color(color_type: ThemeColor) {
  case color_type {
    Accent -> True
    TextBackground -> True
    TextColor -> False
  }
}

pub fn set_theme_color(old_theme: Theme, color_type: ThemeColor, color) {
  set_theme_rgb(old_theme, color_type, default_hex_to_rgb(color))
}

pub fn reset_theme_color(old_theme, color_type) {
  set_theme_rgb(old_theme, color_type, get_default_theme_color(color_type))
}

pub fn reset_all_colors(old_theme) {
  reset_theme_color(old_theme, Accent)
  |> reset_theme_color(TextBackground)
  |> reset_theme_color(TextColor)
}

pub fn is_default(site_theme: Theme, color_type: ThemeColor) {
  get_current_theme_color(site_theme, color_type)
  == get_default_theme_color(color_type)
}

pub fn all_colors_are_default(site_theme) {
  is_default(site_theme, Accent)
  && is_default(site_theme, TextBackground)
  && is_default(site_theme, TextColor)
}

pub fn get_current_theme_color(site_theme: Theme, color_type: ThemeColor) {
  case color_type {
    Accent -> site_theme.accent
    TextBackground -> site_theme.text_background
    TextColor -> site_theme.text_color
  }
}

pub fn get_rgb_arg(hex: String, index: Int) {
  let slice = string.slice(hex, index * 2, 2)
  let arg_missing = string.length(slice) < 2

  case arg_missing {
    True -> Ok(1.0)
    False ->
      slice
      |> int.base_parse(16)
      |> result.map(int.to_float)
      |> result.map(fn(x) { x /. 255.0 })
      |> result.map(fn(x) { float.to_precision(x, 2) })
  }
}

pub fn slice_rgb_arg(hex: String, index: Int) {
  let slice = string.slice(hex, index * 2, 2)
  slice
}

pub fn normalize_hex(hex: String) {
  let clean_hex = string.remove_prefix(hex, "#")

  // guard for shorthand hex
  use <- bool.guard(string.length(clean_hex) >= 6, clean_hex)

  let r = string.slice(clean_hex, 0, 1)
  let g = string.slice(clean_hex, 1, 1)
  let b = string.slice(clean_hex, 2, 1)
  let a = string.slice(clean_hex, 3, 1)

  r <> r <> g <> g <> b <> b <> a <> a
  // TODO: this is stupid
}

pub fn hex_to_rgb(hex: String) {
  let clean_hex = normalize_hex(hex)

  use r <- result.try(get_rgb_arg(clean_hex, 0))
  use g <- result.try(get_rgb_arg(clean_hex, 1))
  use b <- result.try(get_rgb_arg(clean_hex, 2))
  use a <- result.try(get_rgb_arg(clean_hex, 3))

  Ok(niji.Rgb(r, g, b, a))
}

pub fn default_hex_to_rgb(hex: String) {
  hex_to_rgb(hex) |> result.unwrap(niji.rgb(1.0, 0.0, 1.0, 1.0))
}

pub fn default_theme() {
  Theme(
    opaque_background: False,
    text_stroke: True,
    accent: get_default_theme_color(Accent),
    text_background: get_default_theme_color(TextBackground),
    text_color: get_default_theme_color(TextColor),
  )
}

pub fn theme_attributes(site_theme: Theme) {
  theme_color_attributes(site_theme, Accent)
  |> list.append(theme_color_attributes(site_theme, TextBackground))
  |> list.append([
    simple_theme_color_attribute(site_theme, TextColor),
    attribute.style("--text-stroke", case site_theme.text_stroke {
      True -> "var(--text-background-darkest)"
      False -> "rgb(0 0 0 0)"
    }),
    attribute.style(
      "--" <> get_id(TextColor) <> "-inverted",
      get_current_theme_color(site_theme, TextColor)
        |> niji.rgb_to_oklch()
        |> niji.invert_full()
        |> niji.set_alpha(
          1.0
          -. {
            get_current_theme_color(site_theme, TextBackground)
            |> niji.rgb_to_oklch()
            |> niji.luminance()
          },
        )
        |> niji.to_css(),
    ),
  ])
}

pub fn theme_color_attributes(site_theme: Theme, color_type: ThemeColor) {
  let rgb = get_current_theme_color(site_theme, color_type)
  let color_name = get_id(color_type)
  let main_color = niji.rgb_to_oklch(rgb)
  [
    theme_color_attribute(color_name, "color", main_color, 0.0),
    theme_color_attribute(color_name, "light", main_color, 0.07),
    theme_color_attribute(color_name, "lighter", main_color, 0.15),
    theme_color_attribute(color_name, "lightest", main_color, 0.25),
    theme_color_attribute(color_name, "dark", main_color, -0.07),
    theme_color_attribute(color_name, "darker", main_color, -0.15),
    theme_color_attribute(color_name, "darkest", main_color, -0.25),
    attribute.style(
      "--" <> color_name <> "-blacklight",
      main_color
        |> niji.mix(
          niji.rgb_from_ints(180, 163, 255, 1.0) |> niji.rgb_to_oklch(),
          0.6,
        )
        |> niji.to_css(),
    ),
  ]
}

pub fn simple_theme_color_attribute(site_theme: Theme, color_type: ThemeColor) {
  let rgb = get_current_theme_color(site_theme, color_type)
  let color_name = get_id(color_type)
  let main_color = niji.rgb_to_oklch(rgb)
  attribute.style("--" <> color_name, niji.to_css(main_color))
}

pub fn theme_color_attribute(
  color_name: String,
  color_variant: String,
  color: niji.Oklch,
  modulation: Float,
) {
  let modulator = case modulation >. 0.0 {
    True -> niji.lighten
    False -> niji.darken
  }

  attribute.style(
    "--" <> color_name <> "-" <> color_variant,
    modulator(color, float.absolute_value(modulation)) |> niji.to_css(),
  )
}
