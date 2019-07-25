//! A component and system to tie the position of an entity to the position of its physics body

use common::{self, physics};
use nphysics3d;
use specs::{self, Join};

use math::convert;

use output::get_isometry;

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    tie: specs::ReadStorage<'a, physics::PhysicsTiedPosition>,
    physics: specs::ReadStorage<'a, physics::Physics>,
    space: specs::WriteStorage<'a, common::Position>,
    world: specs::WriteExpect<'a, nphysics3d::world::World<::Float>>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (p, s, _) in (&data.physics, &mut data.space, &data.tie).join() {
            let pos = get_isometry(&data.world, p.get_root_handle())
                .translation
                .vector;
            s.0 = convert::to_cgmath_point(pos);
        }
    }
}
