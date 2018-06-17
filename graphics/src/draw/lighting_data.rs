//! A system to collect light info from all light entities

use common::specs::{self, Join, DispatcherBuilder};
use common::cgmath::{self, Point3, Matrix4, SquareMatrix, EuclideanSpace};
use common;

use std::sync::{Arc, Mutex, mpsc};
use std::vec::Drain;

use draw::passes::main::lighting;
use draw::passes::shadow::traits::{LightTransform, PointLightTransform};
use draw::types::AspectRatio;
use draw::components;

pub struct Light<T: LightTransform> {
    pub light: T::ShaderStruct,
    pub shadows: components::ShadowSettings,
    pub transform: T::Transform,
}

impl<T: LightTransform> Light<T> {
    fn new(
        light: T::ShaderStruct,
        shadows: components::ShadowSettings,
        transform: T::Transform,
    ) -> Self {
        Self {
            light,
            shadows,
            transform,
        }
    }
}

/// Data for every light in the world
#[derive(Default)]
pub struct LightingData {
    dir_lights: Vec<Light<components::DirectionalLight>>,
    point_lights: Vec<Light<components::PointLight>>,
    spot_lights: Vec<Light<components::SpotLight>>,
}

impl LightingData {
    pub fn take_dir_lights(&mut self) -> Drain<Light<components::DirectionalLight>> {
        self.dir_lights.drain(0..)
    }

    pub fn take_point_lights(&mut self) -> Drain<Light<components::PointLight>> {
        self.point_lights.drain(0..)
    }

    pub fn take_spot_lights(&mut self) -> Drain<Light<components::SpotLight>> {
        self.spot_lights.drain(0..)
    }

    pub fn reset_dir_lights(&mut self) {
        for _ in 0..lighting::MAX_DIR_LIGHTS {
            self.dir_lights.push(Light::new(
                    Default::default(),
                    components::ShadowSettings::Disabled,
                    Matrix4::identity()));
        }
    }
    
    pub fn reset_point_lights(&mut self) {
        for _ in 0..lighting::MAX_POINT_LIGHTS {
            self.point_lights.push(Light::new(
                    Default::default(),
                    components::ShadowSettings::Disabled,
                    PointLightTransform {
                        matrices: [Matrix4::identity(); 6],
                        light_pos: Point3::origin(),
                        far_plane: 1.0,
                    }));
        }
    }
    
    pub fn reset_spot_lights(&mut self) {
        for _ in 0..lighting::MAX_SPOT_LIGHTS {
            self.spot_lights.push(Light::new(
                    Default::default(),
                    components::ShadowSettings::Disabled,
                    Matrix4::identity()));
        }
    }
}

pub struct System {
    aspect_ratio_point: (AspectRatio, mpsc::Receiver<AspectRatio>),
    aspect_ratio_spot: (AspectRatio, mpsc::Receiver<AspectRatio>),
}

impl System {
    fn update_shadow_map_aspect_ratios(&mut self) {
        let update = |pair: &mut (AspectRatio, mpsc::Receiver<AspectRatio>)| if let Ok(a) =
            pair.1.try_recv()
        {
            pair.0 = a;
        };

        update(&mut self.aspect_ratio_point);
        update(&mut self.aspect_ratio_spot);
    }
}

impl<'a> specs::System<'a> for System {
    type SystemData = SystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let mut light_info = data.light_info.lock().unwrap();

        self.update_shadow_map_aspect_ratios();

        // Clear all lights
        light_info.dir_lights.clear();
        light_info.point_lights.clear();
        light_info.spot_lights.clear();

        // Collect all directional light entities
        for (l, d, s) in (&data.dir_light, &data.direction, &data.space).join() {
            let dir: cgmath::Vector3<f32> = cgmath::Vector3::from(*d).cast();

            let light = lighting::DirectionalLight::from_components(*l, dir.into());

            let transform = l.get_light_space_transform((s.0.cast(), dir));

            light_info.dir_lights.push(
                Light::new(light, l.shadows, transform),
            );
        }

        // Collect all point light entities
        for (l, s) in (&data.point_light, &data.space).join() {
            let pos: [f32; 3] = s.0.cast().into();
            let light = lighting::PointLight::from_components(*l, pos);

            let transform = l.get_light_space_transform((s.0.cast(), self.aspect_ratio_point.0));

            light_info.point_lights.push(Light::new(
                light,
                l.shadows,
                transform,
            ));
        }

        // Collect all spot light entities
        for (l, d, s) in (&data.spot_light, &data.direction, &data.space).join() {
            let pos: [f32; 3] = s.0.cast().into();
            let dir: cgmath::Vector3<f32> = cgmath::Vector3::from(*d).cast();
            let light = lighting::SpotLight::from_components(*l, pos, dir.into());

            let transform =
                l.get_light_space_transform((s.0.cast(), dir, self.aspect_ratio_spot.0));

            light_info.spot_lights.push(Light::new(
                light,
                l.shadows,
                transform,
            ));
        }
    }
}

#[derive(SystemData)]
pub struct SystemData<'a> {
    light_info: specs::FetchMut<'a, Arc<Mutex<LightingData>>>,
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
) -> (DispatcherBuilder<'a, 'b>, mpsc::Sender<AspectRatio>, mpsc::Sender<AspectRatio>) {

    // Add resources
    world.add_resource(Arc::new(Mutex::new(LightingData::default())));

    let (point_send, point_recv) = mpsc::channel();
    let (spot_send, spot_recv) = mpsc::channel();

    // Initialize systems
    let system = System {
        aspect_ratio_point: (Default::default(), point_recv),
        aspect_ratio_spot: (Default::default(), spot_recv),
    };

    // Add systems
    let dispatcher = dispatcher.add(system, "light-info", &[]);

    (dispatcher, point_send, spot_send)
}
