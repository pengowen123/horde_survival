/// A system to update the shader parameter representing the rotation of an entity's model

use specs::{self, Join};
use common::{self, cgmath};

/// A 3D rotation
pub type Rotation = cgmath::Matrix4<f32>;

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    direction: specs::ReadStorage<'a, common::Direction>,
    param: specs::WriteStorage<'a, super::ShaderParam>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (d, p) in (&data.direction, &mut data.param).join() {
            p.rotation = cgmath::Quaternion::from_sv(d.0.s as f32, d.0.v.cast().unwrap()).into();
        }
    }
}
