//! A system to collect light info from all light entities

use specs::{self, Join, DispatcherBuilder};
use cgmath;

use super::pipeline::main::lighting;
use super::components;
use world::components::{Direction, Spatial};

/// Data for every light in the world
#[derive(Default)]
pub struct LightingData {
    dir_lights: [lighting::DirectionalLight; lighting::MAX_LIGHTS],
    point_lights: [lighting::PointLight; lighting::MAX_LIGHTS],
    spot_lights: [lighting::SpotLight; lighting::MAX_LIGHTS],
}

impl LightingData {
    pub fn dir_lights(&self) -> &[lighting::DirectionalLight] {
        &self.dir_lights
    }
    pub fn point_lights(&self) -> &[lighting::PointLight] {
        &self.point_lights
    }
    pub fn spot_lights(&self) -> &[lighting::SpotLight] {
        &self.spot_lights
    }
}

pub struct System;

#[derive(SystemData)]
pub struct SystemData<'a> {
    light_info: specs::FetchMut<'a, LightingData>,
    dir_light: specs::ReadStorage<'a, components::DirectionalLight>,
    point_light: specs::ReadStorage<'a, components::PointLight>,
    spot_light: specs::ReadStorage<'a, components::SpotLight>,
    // Other light properties
    direction: specs::ReadStorage<'a, Direction>,
    space: specs::ReadStorage<'a, Spatial>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = SystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut light_info = data.light_info;

        // Disable all lights (essentially removing them)
        for light in &mut light_info.dir_lights {
            light.enabled = 0;
        }

        for light in &mut light_info.point_lights {
            light.enabled = 0;
        }

        for light in &mut light_info.spot_lights {
            light.enabled = 0;
        }

        // Collect all directional light entities
        for (i, (l, d)) in (&data.dir_light, &data.direction).join().enumerate() {
            if i >= lighting::MAX_LIGHTS {
                warn!(
                    "Maximum directional lights reached: {}",
                    lighting::MAX_LIGHTS
                );
                break;
            }

            let dir: [f32; 3] = (d.0 * cgmath::Vector3::unit_z()).cast().into();
            let direction = [dir[0], dir[1], dir[2], 0.0];

            light_info.dir_lights[i] = lighting::DirectionalLight {
                direction,
                ambient: l.color.ambient,
                diffuse: l.color.diffuse,
                specular: l.color.specular,
                enabled: 1,
                _padding: Default::default(),
                _padding0: Default::default(),
                _padding1: Default::default(),
            };
        }

        // Collect all point light entities
        for (i, (l, s)) in (&data.point_light, &data.space).join().enumerate() {
            if i >= lighting::MAX_LIGHTS {
                warn!("Maximum point lights reached: {}", lighting::MAX_LIGHTS);
                break;
            }

            let pos: [f32; 3] = s.0.cast().into();
            let position = [pos[0], pos[1], pos[2], 1.0];

            light_info.point_lights[i] = lighting::PointLight {
                position,
                ambient: l.color.ambient,
                diffuse: l.color.diffuse,
                specular: l.color.specular,
                constant: l.constant,
                linear: l.linear,
                quadratic: l.quadratic,
                enabled: 1,
            };
        }

        // Collect all spot light entities
        for (i, (l, d, s)) in (&data.spot_light, &data.direction, &data.space)
            .join()
            .enumerate()
        {
            if i >= lighting::MAX_LIGHTS {
                warn!("Maximum spot lights reached: {}", lighting::MAX_LIGHTS);
                break;
            }

            let pos: [f32; 3] = s.0.cast().into();
            let position = [pos[0], pos[1], pos[2], 1.0];

            let dir: [f32; 3] = (d.0 * cgmath::Vector3::unit_z()).cast().into();
            let direction = [dir[0], dir[1], dir[2], 0.0];

            light_info.spot_lights[i] = lighting::SpotLight {
                position,
                direction,
                ambient: l.color.ambient,
                diffuse: l.color.diffuse,
                specular: l.color.specular,
                cos_cutoff: l.cos_cutoff,
                cos_outer_cutoff: l.cos_outer_cutoff,
                enabled: 1,
                _padding: Default::default(),
            };
        }
    }
}

/// Initializes the lighting data system
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {

    // Add resources
    world.add_resource(LightingData::default());

    // Add systems
    dispatcher.add(System, "light-info", &[])
}
