extern crate sfml;
use sfml::graphics::Color;

pub fn are_colors_equal(c1: Color, c2: Color) -> bool {
    c1.red() == c2.red() && c1.green() == c2.green() && c1.blue() == c2.blue()
}
