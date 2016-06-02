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
    position: Vector2f,
    pub clock: Clock,
    cleanup: Vec<usize>

}

impl<'a> ParticleManager<'a> {
    pub fn new() -> Self {
        ParticleManager {
            particles: vec![],
            position: Vector2f::new(0., 0.),
            clock: Clock::new(),
            cleanup: vec![]
        }
    }

    pub fn update(&mut self, dt: f32) {
        for (i, p) in self.particles.iter_mut().enumerate() {
            if p.shape.get_scale().x <= 0.05 {
                self.cleanup.push(i);
            }
            p.shape.move2f(p.direction.x * dt, p.direction.y * dt);
            p.shape.rotate(90. * dt);
            p.shape.scale2f(0.95, 0.95);
        }
        for i in self.cleanup.iter() {
            self.particles.remove(*i);
        }
        self.cleanup.clear();
    }

    pub fn set_position(&mut self, position: &Vector2f) {
        self.position = *position;
    }

    fn create_particle_shape(position: &Vector2f, color: &Color) -> RectangleShape<'a> {
        let mut shape = RectangleShape::new().unwrap();
        shape.set_position(position);
        shape.set_size(&Vector2f::new(20., 20.));
        shape.set_origin(&Vector2f::new(10., 10.));
        shape.set_fill_color(color);
        shape
    }

    pub fn spawn_random_particle(&mut self, color: &Color) {
        let shape = ParticleManager::create_particle_shape(&self.position, color);
        let direction = Vector2f::new(rand::thread_rng().gen_range(-400, 400) as f32, rand::thread_rng().gen_range(-500, -200) as f32);
        self.particles.push(Particle::new(shape, direction));
    }

    pub fn spawn_directed_particle(&mut self, color: &Color, dir: &Vector2f) {
        let shape = ParticleManager::create_particle_shape(&self.position, color);
        self.particles.push(Particle::new(shape, *dir));
    }
}
