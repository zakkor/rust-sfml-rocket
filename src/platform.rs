extern crate sfml;
use sfml::graphics::*;

#[derive(PartialEq)]
pub enum PlatformType {
    Static,
    Moving
}

#[derive(PartialEq)]
pub enum MoveDirection {
    Left,
    Right
}

pub struct Platform<'a> {
    pub shape: RectangleShape<'a>,
    pub plat_type: PlatformType,
    pub move_dir: MoveDirection,
    pub move_speed: f32
}

impl<'a> Platform<'a> {
    pub fn new(shape: RectangleShape<'a>, plat_type: PlatformType, move_speed: f32) -> Self {
        Platform {
            shape: shape,
            plat_type: plat_type,
            move_dir: MoveDirection::Right,
            move_speed: move_speed
        }
    }

    pub fn move_platform(&mut self) {
        if self.plat_type == PlatformType::Moving {
            if self.move_dir == MoveDirection::Right {
                if self.shape.get_position().x + self.shape.get_local_bounds().width >= 1280. {
                    self.move_dir = MoveDirection::Left;
                }
                else {
                    self.shape.move2f( self.move_speed, 0.);
                }
            }
            else {
                if self.shape.get_position().x <= 0. {
                    self.move_dir = MoveDirection::Right;
                }
                else {
                    self.shape.move2f( -self.move_speed, 0.);
                }
            }
        }
    }
}
