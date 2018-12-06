//! A system for updating particles and particle sources

use common::{self, config, shred};
use common::graphics::ParticleSource;
use common::specs::{self, Join};
use window::window_event;

pub struct System {
    reader_id: window_event::ReaderId,
}

impl System {
    fn new(resources: &mut shred::Resources) -> Self {
        let reader_id = {
            let mut event_channel = resources.fetch_mut::<window_event::EventChannel>();
            event_channel.register_reader()
        };

        Self {
            reader_id,
        }
    }
}

#[derive(SystemData)]
pub struct SystemData<'a> {
    delta: specs::ReadExpect<'a, common::Delta>,
    particle_source: specs::WriteStorage<'a, ParticleSource>,
    position: specs::ReadStorage<'a, common::Position>,
    event_channel: specs::ReadExpect<'a, window_event::EventChannel>,
    config: specs::ReadExpect<'a, config::Config>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = SystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mut updated_particles_option = None;

        for e in data.event_channel.read(&mut self.reader_id) {
            match *e {
                window_event::Event::ConfigChanged(window_event::ChangedConfig::Graphics) => {
                    updated_particles_option = Some(data.config.graphics.particles);
                }
                _ => {},
            }
        }

        let dt = data.delta.to_float() as f32;
        for (pos, source) in (&data.position, &mut data.particle_source).join() {
            match updated_particles_option {
                Some(true) => source.enable(),
                Some(false) => source.disable(),
                None => {},
            }

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

    dispatcher.with(System::new(&mut world.res), "particles", &[])
}
