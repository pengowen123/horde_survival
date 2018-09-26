//! Types and a system for the CPU side of particle simulation

use common::specs::{self, Join};
use common::{self, cgmath};

/// An individual particle
#[derive(Clone, Debug)]
pub struct Particle {
    color: [f32; 4],
    position: cgmath::Point3<f32>,
    velocity: cgmath::Vector3<f32>,
    gravity: f32,
    lifetime: ::Float,
}

impl Particle {
    /// Returns a new `Particle`
    ///
    /// `lifetime` should be in seconds.
    pub fn new(
        color: [f32; 4],
        position: cgmath::Point3<f32>,
        velocity: cgmath::Vector3<f32>,
        gravity: f32,
        lifetime: ::Float,
    ) -> Self {
        Self {
            color,
            position,
            velocity,
            gravity,
            lifetime,
        }
    }

    /// Returns the color of this `Particle`
    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    /// Returns the position of this `Particle`
    pub fn position(&self) -> cgmath::Point3<f32> {
        self.position
    }

    /// Returns the remaining lifetime of this `Particle`
    pub fn lifetime(&self) -> ::Float {
        self.lifetime
    }

    /// Updates this `Particle` based on the provided delta time
    fn update(&mut self, dt: f32) {
        if self.lifetime > 0.0 {
            self.velocity.z += self.gravity * dt;
            self.position += self.velocity * dt;
            self.lifetime -= ::Float::from(dt);
        }
    }
}

/// A function that generates a new particle from the particle source's position
pub type SpawnParticleFn = Box<FnMut(&cgmath::Point3<f32>) -> Particle + Send + Sync>;

/// A component that causes an entity to produce particles
pub struct ParticleSource {
    particles: Vec<Particle>,
    spawn_rate: f32,
    spawn_dt_accumulator: f32,
    spawn_fn: SpawnParticleFn,
    enabled: bool,
}

impl ParticleSource {
    /// Returns a new `ParticleSource`
    ///
    /// `spawn_rate` determines how many new particles will be spawned per second
    /// `spawn_fn` is called to determine parameters of new particles
    pub fn new<F>(max_particles: usize, spawn_rate: f32, spawn_fn: F) -> Self
    where
        F: Into<SpawnParticleFn>,
    {
        Self {
            particles: Vec::with_capacity(max_particles),
            spawn_rate,
            spawn_dt_accumulator: 0.0,
            spawn_fn: spawn_fn.into(),
            enabled: true,
        }
    }

    fn update(&mut self, dt: f32, source_position: &cgmath::Point3<f32>) {
        // Spawn new particles
        self.spawn_dt_accumulator += dt * self.spawn_rate;
        let new_particles = self.spawn_dt_accumulator.floor();
        self.spawn_dt_accumulator -= new_particles;

        let mut new_particles = new_particles as usize;

        while new_particles > 0 && self.particles.len() < self.particles.capacity() {
            new_particles -= 1;

            self.particles.push((self.spawn_fn)(source_position));
        }

        for i in first_unused_particles(&self.particles, new_particles) {
            self.particles[i] = (self.spawn_fn)(source_position);
        }

        // Update particles
        for particle in &mut self.particles {
            particle.update(dt);
        }
    }

    /// Disables this `ParticleSource`
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Enables this `ParticleSource`
    pub fn enable(&mut self) {
        self.enabled = true;
    }
}

impl specs::Component for ParticleSource {
    type Storage = specs::DenseVecStorage<Self>;
}

/// Returns the indices of the first `n` particles from `list` that have a lifetime less than or
/// equal to zero
///
/// Returns fewer than `n` indices if there aren't `n` unused particles in the list.
fn first_unused_particles<'a>(list: &'a [Particle], n: usize) -> Vec<usize> {
    list.iter()
        .enumerate()
        .filter_map(|(i, p)| if p.lifetime() <= 0.0 { Some(i) } else { None })
        .take(n)
        .collect()
}

pub struct System;

#[derive(SystemData)]
pub struct SystemData<'a> {
    delta: specs::ReadExpect<'a, common::Delta>,
    particle_source: specs::WriteStorage<'a, ParticleSource>,
    position: specs::ReadStorage<'a, common::Position>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = SystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let dt = data.delta.to_float() as f32;
        for (pos, source) in (&data.position, &mut data.particle_source).join() {
            source.update(dt, &pos.0.cast().unwrap());
        }
    }
}

/// Initializes particle-related components and systems
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: specs::DispatcherBuilder<'a, 'b>,
) -> specs::DispatcherBuilder<'a, 'b> {
    world.register::<ParticleSource>();

    dispatcher.with(System, "particles", &[])
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::cgmath::{self, EuclideanSpace, Zero};

    fn spawn_particle(pos: &cgmath::Point3<f32>) -> Particle {
        Particle::new([1.0; 4], *pos, cgmath::Vector3::unit_z(), -1.0, 100.0)
    }

    #[test]
    fn test_particle_spawn_rate() {
        let mut source = ParticleSource::new(100, 2.0, Box::new(spawn_particle) as SpawnParticleFn);
        let origin = cgmath::Point3::origin();

        source.update(0.25, &origin);
        assert_eq!(source.particles.len(), 0);

        source.update(0.25, &origin);
        assert_eq!(source.particles.len(), 1);

        source.update(0.25, &origin);
        assert_eq!(source.particles.len(), 1);

        source.update(0.25, &origin);
        assert_eq!(source.particles.len(), 2);
    }

    #[test]
    fn test_particle_update() {
        let mut source = ParticleSource::new(1, 1.0, Box::new(spawn_particle) as SpawnParticleFn);
        let origin = cgmath::Point3::origin();

        source.update(1.0, &origin);
        assert_eq!(source.particles.len(), 1);
        assert_eq!(source.particles[0].velocity, cgmath::Vector3::zero());
        source.update(1.0, &origin);
        assert_eq!(
            source.particles[0].position,
            cgmath::Point3::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn test_first_unused_particles() {
        let particle = |lifetime| Particle {
            lifetime,
            ..spawn_particle(&cgmath::Point3::origin())
        };
        let list = [
            particle(1.0),
            particle(0.5),
            particle(0.0),
            particle(-0.5),
            particle(-1.0),
        ];

        assert_eq!(first_unused_particles(&list, 2), vec![2, 3]);
        assert_eq!(first_unused_particles(&list, 3), vec![2, 3, 4]);
        assert_eq!(first_unused_particles(&list, 4), vec![2, 3, 4]);
    }
}
