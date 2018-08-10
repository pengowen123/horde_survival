//! Pipeline declarations for shadows

pub mod directional;

use specs::{self, Join};
use cgmath;

use std::sync::{Arc, Mutex};

use draw::components;

/// A matrix that goes from model space to light space (a View * Projection matrix)
#[derive(Clone, Copy, Debug)]
pub struct LightSpaceMatrix(pub cgmath::Matrix4<f32>);

impl LightSpaceMatrix {
    pub fn from_components(
        projection: cgmath::Ortho<f32>,
        position: cgmath::Point3<f32>,
        direction: cgmath::Vector3<f32>,
    ) -> Self {
        let projection: cgmath::Matrix4<_> = projection.into();
        let view =
            cgmath::Matrix4::look_at(position, position + direction, cgmath::Vector3::unit_z());

        let light_space_matrix = projection * view;

        LightSpaceMatrix(light_space_matrix)
    }
}

/// An optional directional light shadow source
#[derive(Clone, Copy, Debug)]
pub struct DirShadowSource(Option<LightSpaceMatrix>);

impl DirShadowSource {
    pub fn new(light_space_matrix: LightSpaceMatrix) -> Self {
        DirShadowSource(Some(light_space_matrix))
    }

    /// Returns a new `DirShadowSource` that is `None`
    pub fn new_none() -> Self {
        DirShadowSource(None)
    }

    /// Returns the light space matrix of this shadow source if the shadow source exists
    pub fn light_space_matrix(&self) -> Option<cgmath::Matrix4<f32>> {
        self.0.map(|lsm| lsm.0)
    }

    /// Removes the directional shadow source
    pub fn clear(&mut self) {
        self.0 = None;
    }
}

/// This system updates the shadow source resources with properties from light entities with shadows
/// enabled
pub struct ShadowSourceSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    dir_light: specs::ReadStorage<'a, components::DirectionalLight>,
    dir_shadow_source: specs::WriteExpect<'a, Arc<Mutex<DirShadowSource>>>,
}
impl<'a> specs::System<'a> for ShadowSourceSystem {
    type SystemData = Data<'a>;
    
    fn run(&mut self, data: Self::SystemData) {
        for (i, dir_light) in (&data.dir_light).join().enumerate() {
            if i > 0 {
                println!("found more than one directional shadow source");
            }

            if let Some(light_space_matrix) = dir_light.shadows {
                *data.dir_shadow_source.lock().unwrap() = DirShadowSource::new(light_space_matrix);
            }
        }
    }
}
