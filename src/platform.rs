extern crate sfml;
use sfml::graphics::*;

#[derive(PartialEq)]
pub enum PlatformType {
    Static,
    Moving,
}

#[derive(PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
}

pub struct Platform<'a> {
    pub shape: RectangleShape<'a>,
    pub plat_type: PlatformType,
    pub move_dir: MoveDirection,
    pub move_speed: f32,
}

impl<'a> Platform<'a> {
    pub fn new(shape: RectangleShape<'a>, plat_type: PlatformType, move_speed: f32) -> Self {
        Platform {
            shape,
            plat_type,
            move_dir: MoveDirection::Right,
            move_speed,
        }
    }

    pub fn move_platform(&mut self, multiplier: &f32) {
        if self.plat_type == PlatformType::Moving {
            if self.move_dir == MoveDirection::Right {
                if self.shape.position().x + self.shape.local_bounds().width >= 1280. {
                    self.move_dir = MoveDirection::Left;
                } else {
                    self.shape.move_((self.move_speed * multiplier, 0.));
                }
            } else if self.shape.position().x <= 0. {
                self.move_dir = MoveDirection::Right;
            } else {
                self.shape.move_((-self.move_speed * multiplier, 0.));
            }
        }
    }
}
