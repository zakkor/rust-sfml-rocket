use std::collections::HashMap;
use std::hash::Hash;
extern crate sfml;
use sfml::graphics::Texture;

pub trait Resource: Sized {
    fn new_from_file(filename: &str) -> Option<Self>;
}

impl Resource for Texture {
    fn new_from_file(filename: &str) -> Option<Self> {
        Texture::new_from_file(filename)
    }
}

pub struct ResourceManager<I, R> {
    resource_map: HashMap<I, Box<R>>
}

impl<I: Eq + Hash, R: Resource> ResourceManager<I, R> {
    pub fn new() -> Self {
        ResourceManager {
            resource_map: HashMap::<I, Box<R>>::new()
        }
    }

    pub fn load(&mut self, identifier: I, filename: & str) {
        let resource = R::new_from_file(filename).unwrap();
        self.resource_map.insert(identifier, Box::new(resource));
    }

    pub fn get(&self, identifier: I) -> &Box<R> {
        match self.resource_map.get(&identifier) {
            Some(resource) => resource,
            None => panic!("Tried to access nonexistant index in resource map")
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureIdentifiers {
    Nebula
}

pub type TextureManager = ResourceManager<TextureIdentifiers, Texture>;
