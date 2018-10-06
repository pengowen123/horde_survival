//! A system for updating particles and particle sources

use common;
use common::graphics::ParticleSource;
use common::specs::{self, Join};

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
