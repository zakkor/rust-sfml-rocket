extern crate sfml;
use sfml::graphics::*;

#[derive(PartialEq)]
pub enum PlatformType {
    Static,
    Moving
}

pub struct Platform<'a> {
    pub shape: RectangleShape<'a>,
    pub plat_type: PlatformType
}

impl<'a> Platform<'a> {
    pub fn new(shape: RectangleShape<'a>, plat_type: PlatformType) -> Self {
        Platform {
            shape: shape,
            plat_type: plat_type
        }
    }
}
