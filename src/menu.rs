extern crate sfml;
use sfml::graphics::*;
use sfml::system::Vector2f;

pub enum ButtonType {
    Quit,
    Resume,
}

pub struct Button<'a> {
    pub button_type: ButtonType,
    pub text: Text<'a>,
}

impl<'a> Button<'a> {
    pub fn new(font: &'a Font, button_type: ButtonType, pos: &Vector2f) -> Self {
        let mut text = Text::new().unwrap();
        text.set_font(font);
        text.set_color(&Color::white());
        text.set_character_size(50);


        match button_type {
            ButtonType::Quit => {
                text.set_string("QUIT");
            },
            ButtonType::Resume => {
                text.set_string("RESUME");
            },
        }

//        let text_size = text.get_global_bounds();

//        println!("{:?}", text_size);

        text.set_position(pos);

        Button {
            button_type: button_type,
            text: text,
        }
    }
}

pub struct Menu<'a> {
    pub buttons: Vec<Button<'a>>
}


