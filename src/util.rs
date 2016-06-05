extern crate sfml;
use sfml::graphics::Color;

pub fn are_colors_equal(c1: &Color, c2: &Color) -> bool {
    if c1.0.red == c2.0.red &&
        c1.0.green == c2.0.green &&
        c1.0.blue == c2.0.blue {
            true
        }
    else {
        false
    }
}
