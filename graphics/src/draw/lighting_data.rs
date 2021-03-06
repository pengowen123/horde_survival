//! A system to collect light info from all light entities

use common;
use common::cgmath;
use common::specs::{self, DispatcherBuilder, Join};

use std::sync::{Arc, Mutex};
use std::vec::Drain;

use draw::components;
use draw::passes::main::lighting;

/// Data for every light in the world
#[derive(Default)]
pub struct LightingData {
    dir_lights: Vec<lighting::DirectionalLight>,
    point_lights: Vec<lighting::PointLight>,
    spot_lights: Vec<lighting::SpotLight>,
}

impl LightingData {
    pub fn take_dir_lights(&mut self) -> Drain<lighting::DirectionalLight> {
        self.dir_lights.drain(..)
    }

    pub fn take_point_lights(&mut self) -> Drain<lighting::PointLight> {
        self.point_lights.drain(..)
    }

    pub fn take_spot_lights(&mut self) -> Drain<lighting::SpotLight> {
        self.spot_lights.drain(..)
    }

    pub fn reset_dir_lights(&mut self) {
        for _ in 0..lighting::MAX_DIR_LIGHTS {
            self.dir_lights.push(lighting::DirectionalLight {
                direction: [1.0, 0.0, 0.0, 0.0],
                ambient: [0.0; 4],
                diffuse: [0.0; 4],
                specular: [0.0; 4],
                has_shadows: 0.0,
                enabled: 0.0,
                _padding: Default::default(),
            });
        }
    }

    pub fn reset_point_lights(&mut self) {
        for _ in 0..lighting::MAX_POINT_LIGHTS {
            self.point_lights.push(lighting::PointLight {
                position: [0.0, 0.0, 0.0, 1.0],
                ambient: [1.0; 4],
                diffuse: [1.0; 4],
                specular: [1.0; 4],
                constant: 1.0,
                linear: 1.0,
                quadratic: 1.0,
                enabled: 0.0,
            });
        }
    }

    pub fn reset_spot_lights(&mut self) {
        for _ in 0..lighting::MAX_SPOT_LIGHTS {
            self.spot_lights.push(lighting::SpotLight {
                position: [0.0, 0.0, 0.0, 1.0],
                ambient: [1.0; 4],
                diffuse: [1.0; 4],
                specular: [1.0; 4],
                constant: 1.0,
                linear: 1.0,
                quadratic: 1.0,
                cos_cutoff: 0.5,
                cos_outer_cutoff: 1.0,
                direction: [1.0, 0.0, 0.0, 0.0],
                enabled: 0.0,
                _padding: Default::default(),
            });
        }
    }
}

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = SystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut light_info = data.light_info.lock().unwrap();

        // Clear all lights (reset them to default values so the shaders can use them without
        // consequence)
        light_info.reset_dir_lights();
        light_info.reset_point_lights();
        light_info.reset_spot_lights();

        // Collect all directional light entities
        for (i, (l, d)) in (&data.dir_light, &data.direction).join().enumerate() {
            let dir: cgmath::Vector3<f32> = cgmath::Vector3::from(*d).cast().unwrap();

            let light =
                lighting::DirectionalLight::from_components(*l, dir.into(), l.shadows.is_some());

            light_info.dir_lights[i] = light;
        }

        // Collect all point light entities
        for (i, (l, s)) in (&data.point_light, &data.space).join().enumerate() {
            let pos: [f32; 3] = s.0.cast().unwrap().into();
            let light = lighting::PointLight::from_components(*l, pos);

            light_info.point_lights[i] = light;
        }

        // Collect all spot light entities
        for (i, (l, d, s)) in (&data.spot_light, &data.direction, &data.space)
            .join()
            .enumerate()
        {
            let pos: [f32; 3] = s.0.cast().unwrap().into();
            let dir: cgmath::Vector3<f32> = cgmath::Vector3::from(*d).cast().unwrap();
            let light = lighting::SpotLight::from_components(*l, pos, dir.into());

            light_info.spot_lights[i] = light;
        }

        // Put the light with shadows enabled at index 0 of the list (required by the shaders)
        light_info
            .dir_lights
            .sort_by_key(|light| light.has_shadows as i32);
    }
}

#[derive(SystemData)]
pub struct SystemData<'a> {
    light_info: specs::WriteExpect<'a, Arc<Mutex<LightingData>>>,
    dir_light: specs::ReadStorage<'a, components::DirectionalLight>,
    point_light: specs::ReadStorage<'a, components::PointLight>,
    spot_light: specs::ReadStorage<'a, components::SpotLight>,
    // Other light properties
    direction: specs::ReadStorage<'a, common::Direction>,
    space: specs::ReadStorage<'a, common::Position>,
}

/// Initializes the lighting data system
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    // Add resources
    world.add_resource(Arc::new(Mutex::new(LightingData::default())));

    // Initialize systems
    let system = System;

    // Add systems
    dispatcher.with(system, "light-info", &[])
}
