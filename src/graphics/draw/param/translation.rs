/// A system to update the shader parameter representing the translation of an entity's model
///
/// Gets the translation data from the entity's position

use specs::{self, Join};
use cgmath::{self, EuclideanSpace};
use common;

/// A 3D translation
pub type Translation = cgmath::Matrix4<f32>;

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    space: specs::ReadStorage<'a, common::Position>,
    param: specs::WriteStorage<'a, super::ShaderParam>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (s, p) in (&data.space, &mut data.param).join() {
            p.translation = cgmath::Matrix4::from_translation(s.0.cast::<f32>().to_vec());
        }
    }
}
