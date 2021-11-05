extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;

pub struct Score<'a> {
    pub number: u32,
    pub text: Text<'a>,
}

impl<'a> Score<'a> {
    pub fn new() -> Self {
        let mut text = Text::default();
        text.set_string("0");
        text.set_position(Vector2f::new(1280. / 2., 25.));
        text.set_character_size(30);
        text.set_fill_color(Color::WHITE);
        Score { number: 0, text }
    }
    pub fn reset(&mut self) {
        self.text.set_string("0");
        self.text.set_position(Vector2f::new(1280. / 2., 25.));
        self.text.set_character_size(30);
        self.text.set_fill_color(Color::WHITE);
        self.number = 0;
    }
}
