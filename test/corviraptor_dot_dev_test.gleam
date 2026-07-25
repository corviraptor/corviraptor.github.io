import gleeunit
import niji
import theme

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn hex_arg_test() {
  let color = "FF0064"
  let arg = theme.get_rgb_arg(color, 2)

  assert arg == Ok(0.39)
}

pub fn normalize_hex_test() {
  let color = "#F06"
  let rgb = theme.normalize_hex(color)

  assert rgb == "FF0066"
}

pub fn hex_to_rgb_test() {
  let color = "#FF0064"
  let rgb = theme.hex_to_rgb(color)

  assert rgb == Ok(niji.rgb(1.0, 0.0, 0.39, 1.0))
}
