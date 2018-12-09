//! Types for particle simulation

use cgmath;
use specs;
use gfx::{self, handle};

/// The maximum number of particles for a single particle source
pub const MAX_PARTICLES: usize = 512;

/// A function that generates a new particle from the particle source's position
pub type SpawnParticleFn = Box<FnMut(&cgmath::Point3<f32>) -> Particle + Send + Sync>;

/// An individual particle
#[derive(Clone, Debug)]
pub struct Particle {
    alpha: f32,
    alpha_falloff: f32,
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
        alpha: f32,
        alpha_falloff: f32,
        position: cgmath::Point3<f32>,
        velocity: cgmath::Vector3<f32>,
        gravity: f32,
        lifetime: ::Float,
    ) -> Self {
        Self {
            alpha,
            alpha_falloff,
            position,
            velocity,
            gravity,
            lifetime,
        }
    }

    /// Returns the alpha of this `Particle`
    pub fn alpha(&self) -> f32 {
        self.alpha
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
            self.alpha = (self.alpha - self.alpha_falloff * dt).max(0.0);
        }
    }
}

/// A component that causes an entity to produce particles
pub struct ParticleSource<R: gfx::Resources> {
    particles: Vec<Particle>,
    spawn_rate: f32,
    spawn_dt_accumulator: f32,
    spawn_fn: SpawnParticleFn,
    texture: handle::ShaderResourceView<R, [f32; 4]>,
    enabled: bool,
}

quick_error! {
    #[derive(Debug)]
    pub enum ParticleSourceError {
        MaxParticles(requested: usize, max: usize) {
            display("The requested maximum particle count was too high: {} (max {})",
                    requested, max)
        }
    }
}

impl<R: gfx::Resources> ParticleSource<R> {
    /// Returns a new `ParticleSource`
    ///
    /// Returns `Err` if `max_particles` is greater than `MAX_PARTICLES`.
    ///
    /// `spawn_rate` determines how many new particles will be spawned per second
    /// `spawn_fn` is called to determine parameters of new particles
    pub fn new<F>(
        max_particles: usize,
        spawn_rate: f32,
        spawn_fn: F,
        texture: handle::ShaderResourceView<R, [f32; 4]>,
    ) -> Result<Self, ParticleSourceError>
    where
        F: Into<SpawnParticleFn>,
    {
        if max_particles > MAX_PARTICLES {
            Err(ParticleSourceError::MaxParticles(
                max_particles,
                MAX_PARTICLES,
            ))
        } else {
            Ok(Self {
                particles: Vec::with_capacity(max_particles),
                spawn_rate,
                spawn_dt_accumulator: 0.0,
                spawn_fn: spawn_fn.into(),
                texture,
                enabled: true,
            })
        }
    }

    /// Updates this `ParticleSource` and all of its `Particles`
    pub fn update(&mut self, dt: f32, source_position: &cgmath::Point3<f32>) {
        if self.enabled {
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
        }

        // Update particles
        for particle in &mut self.particles {
            particle.update(dt);
        }
    }

    /// Disables this `ParticleSource`
    ///
    /// This prevents new particles from being produced, while still simulating the currently active
    /// ones.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Enables this `ParticleSource`
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Returns the list of particles from this `ParticleSource`
    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    // Returns the texture of this `ParticleSource`
    pub fn texture(&self) -> &handle::ShaderResourceView<R, [f32; 4]> {
        &self.texture
    }
}

impl<R: gfx::Resources> specs::Component for ParticleSource<R> {
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

    #[test]
    fn test_disable_enable() {
        let mut source = ParticleSource::new(100, 2.0, Box::new(spawn_particle) as SpawnParticleFn);
        let origin = cgmath::Point3::origin();

        source.disable();

        source.update(1.0, &origin);
        assert_eq!(source.particles.len(), 0);

        source.enable();

        source.update(1.0, &origin);
        assert_eq!(source.particles.len(), 2);
    }
}
