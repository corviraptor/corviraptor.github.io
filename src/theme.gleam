import gleam/bool
import gleam/float
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import lustre/attribute
import niji

pub type ThemeColorSet {
  ThemeColorSet(accent: niji.Rgb, text_background: niji.Rgb)
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

pub fn default_theme_color_set() {
  ThemeColorSet(
    accent: default_hex_to_rgb("cd125d"),
    text_background: default_hex_to_rgb("302938"),
  )
}

pub fn theme_attributes(color_set: ThemeColorSet) {
  theme_color_attributes("accent-main", color_set.accent)
  |> list.append(theme_color_attributes(
    "text-background",
    color_set.text_background,
  ))
}

pub fn theme_color_attributes(color_name: String, rgb: niji.Rgb) {
  let main_color = niji.rgb_to_oklch(rgb)
  [
    theme_color_attribute(color_name, "color", main_color, 0.0),
    theme_color_attribute(color_name, "light", main_color, 0.1),
    theme_color_attribute(color_name, "lighter", main_color, 0.2),
    theme_color_attribute(color_name, "lightest", main_color, 0.4),
    theme_color_attribute(color_name, "dark", main_color, -0.1),
    theme_color_attribute(color_name, "darker", main_color, -0.25),
    theme_color_attribute(color_name, "darkest", main_color, -0.4),
  ]
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
