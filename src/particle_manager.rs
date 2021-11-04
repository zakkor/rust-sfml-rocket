extern crate rand;
extern crate sfml;

use rand::Rng;
use sfml::graphics::*;
use sfml::system::*;

pub struct Particle<'a> {
    pub shape: RectangleShape<'a>,
    pub direction: Vector2f,
    pub mark_for_explosion: bool,
}

impl<'a> Particle<'a> {
    fn new(shape: RectangleShape<'a>, direction: Vector2f) -> Self {
        Particle {
            shape,
            direction,
            mark_for_explosion: false,
        }
    }
}

pub struct ParticleManager<'a> {
    pub particles: Vec<Particle<'a>>,
    position: Vector2f,
    pub clock: Clock,
    cleanup: Vec<usize>,
}

impl<'a> ParticleManager<'a> {
    pub fn new() -> Self {
        ParticleManager {
            particles: vec![],
            position: Vector2f::new(0., 0.),
            clock: Clock::start(),
            cleanup: vec![],
        }
    }

    pub fn update(&mut self, dt: f32, downwards_speed: f32) {
        for (i, p) in self.particles.iter_mut().enumerate() {
            if p.shape.get_scale().x <= 0.05 {
                self.cleanup.push(i);
            } else if p.mark_for_explosion {
                if p.shape.get_scale().x >= 1.1 {
                    self.cleanup.push(i);
                } else {
                    p.shape.move_((0., downwards_speed));
                    p.shape.scale((1.1, 1.1));
                }
            } else {
                p.shape.move_((p.direction.x * dt, p.direction.y * dt));
                p.shape.rotate(90. * dt);
                p.shape.scale((0.95, 0.95));
            }
        }

        // need reverse iterator, otherwise removing the smaller index particles first will cause a
        // segfault for the bigger index ones
        for i in self.cleanup.iter().rev() {
            self.particles.remove(*i);
        }
        self.cleanup.clear();
    }

    pub fn set_position(&mut self, position: &Vector2f) {
        self.position = *position;
    }

    fn create_particle_shape(position: &Vector2f, color: &Color) -> RectangleShape<'a> {
        let mut shape = RectangleShape::default();
        shape.set_position(*position);
        shape.set_size(Vector2f::new(20., 20.));
        shape.set_origin(Vector2f::new(10., 10.));
        shape.set_fill_color(*color);
        shape
    }

    pub fn spawn_random_particle(&mut self, color: &Color) {
        let shape = ParticleManager::create_particle_shape(&self.position, color);
        let direction = Vector2f::new(
            rand::thread_rng().gen_range(-400..400) as f32,
            rand::thread_rng().gen_range(-500..-200) as f32,
        );
        self.particles.push(Particle::new(shape, direction));
    }

    pub fn spawn_directed_particle(&mut self, color: Color, dir: &Vector2f, is_big: &bool) {
        let mut shape = ParticleManager::create_particle_shape(&self.position, &color);

        if *is_big {
            shape.set_scale((2., 2.));
        }

        self.particles.push(Particle::new(shape, *dir));
    }

    pub fn reset(&mut self) {
        self.particles.clear();
    }
}
