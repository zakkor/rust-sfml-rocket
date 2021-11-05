use std::collections::HashMap;
use std::hash::Hash;
extern crate sfml;
use sfml::{
    graphics::{Font, Texture},
    SfBox,
};

pub trait Resource: Sized {
    fn new_from_file(filename: &str) -> Option<Self>;
}

impl Resource for SfBox<Texture> {
    fn new_from_file(filename: &str) -> Option<Self> {
        Texture::from_file(filename)
    }
}

impl Resource for SfBox<Font> {
    fn new_from_file(filename: &str) -> Option<Self> {
        Font::from_file(filename)
    }
}

pub struct ResourceManager<I, R> {
    resource_map: HashMap<I, Box<R>>,
}

impl<I: Eq + Hash, R: Resource> ResourceManager<I, R> {
    pub fn new() -> Self {
        ResourceManager {
            resource_map: HashMap::<I, Box<R>>::new(),
        }
    }

    pub fn load(&mut self, identifier: I, filename: &str) {
        let resource = R::new_from_file(filename).unwrap();
        self.resource_map.insert(identifier, Box::new(resource));
    }

    pub fn get(&self, identifier: I) -> &R {
        match self.resource_map.get(&identifier) {
            Some(resource) => resource,
            None => panic!("Tried to access nonexistant index in resource map"),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureIdentifiers {
    Nebula,
    Rocket,
}
#[derive(PartialEq, Eq, Hash)]
pub enum FontIdentifiers {
    Arial,
    Joystix,
}

pub type TextureManager = ResourceManager<TextureIdentifiers, SfBox<Texture>>;
pub type FontManager = ResourceManager<FontIdentifiers, SfBox<Font>>;
