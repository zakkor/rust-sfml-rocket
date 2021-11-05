extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;

pub struct Player<'a> {
    pub shape: RectangleShape<'a>,
    pub is_dashing: bool,
    pub dash_clock: Clock,
}

impl<'a> Player<'a> {
    pub fn new() -> Self {
        let mut shape = RectangleShape::default();
        shape.set_size(Vector2f::new(25., 50.));
        shape.set_fill_color(Color::RED);
        shape.set_position(Vector2f::new(1280. / 2., 720. - 200.));
        shape.set_outline_thickness(1.);
        shape.set_outline_color(Color::WHITE);
        shape.set_origin(Vector2f::new(25. / 2., 25.));

        Player {
            shape,
            is_dashing: false,
            dash_clock: Clock::start(),
        }
    }
}
