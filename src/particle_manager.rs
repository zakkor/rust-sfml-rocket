extern crate sfml;
extern crate rand;

use rand::Rng;
use sfml::graphics::*;
use sfml::system::*;

pub struct Particle<'a> {
    pub shape: RectangleShape<'a>,
    pub direction: Vector2f
}

impl<'a> Particle<'a> {
    fn new(shape: RectangleShape<'a>, direction: Vector2f) -> Self {
        Particle {
            shape: shape,
            direction: direction
        }
    }
}

pub struct ParticleManager<'a> {
    pub particles: Vec<Particle<'a>>,
    position: Vector2f

}

impl<'a> ParticleManager<'a> {
    pub fn new() -> Self {
        ParticleManager {
            particles: vec![],
            position: Vector2f::new(0., 0.)
        }
    }

    pub fn update(&mut self, dt: f32) {
        for p in self.particles.iter_mut() {
            // TODO: remove particles that are invisible (too small)
            p.shape.move2f(p.direction.x * dt, p.direction.y * dt);
            p.shape.rotate(90. * dt);
            p.shape.scale2f(0.95, 0.95);
        }
    }

    pub fn set_position(&mut self, position: &Vector2f) {
        self.position = *position;
    }

    pub fn spawn_new_particle(&mut self, color: &Color) {
        let mut shape = RectangleShape::new().unwrap();
        shape.set_position(&self.position);
        shape.set_size(&Vector2f::new(20., 20.));
        shape.set_origin(&Vector2f::new(10., 10.));
        shape.set_fill_color(color);
        let direction = Vector2f::new(rand::thread_rng().gen_range(-300, 300) as f32, rand::thread_rng().gen_range(-300, 300) as f32);
        self.particles.push(Particle::new(shape, direction));
    }
}
