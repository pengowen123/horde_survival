use common::{self, cgmath};
/// A system to update the shader parameter representing the scale of an entity's model
///
/// Gets the scale data from the entity's `Scale` component
use specs::{self, Join};

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    scale: specs::ReadStorage<'a, common::Scale>,
    param: specs::WriteStorage<'a, super::ShaderParam>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (s, p) in (&data.scale, &mut data.param).join() {
            p.set_scale(cgmath::Matrix4::from_scale(s.get()));
        }
    }
}
