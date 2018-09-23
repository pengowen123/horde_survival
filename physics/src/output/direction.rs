//! A component and system to tie the direction of an entity to the direction of its physics body

use nphysics3d::world::World;
use specs::{self, Join};

use common::{self, physics};
use math::convert;

use output::get_isometry;

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    tie: specs::ReadStorage<'a, physics::PhysicsTiedDirection>,
    physics: specs::ReadStorage<'a, physics::Physics>,
    direction: specs::WriteStorage<'a, common::Direction>,
    world: specs::WriteExpect<'a, World<::Float>>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (p, d, _) in (&data.physics, &mut data.direction, &data.tie).join() {
            let quat = get_isometry(&data.world, p.get_root_handle()).rotation;
            d.0 = convert::to_cgmath_quaternion(quat);
        }
    }
}
