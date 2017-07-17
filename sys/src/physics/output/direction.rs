//! A component and system to tie the direction of an entity to the direction of its physics body

use specs::{self, Join};

use world;
use physics;
use math::convert;

/// This component acts as a flag to enable the overwriting of an entity's direction with the
/// direction of its physics body
pub struct PhysicsTiedDirection;

impl specs::Component for PhysicsTiedDirection {
    type Storage = specs::VecStorage<Self>;
}

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    tie: specs::ReadStorage<'a, PhysicsTiedDirection>,
    physics: specs::ReadStorage<'a, physics::components::Physics>,
    direction: specs::WriteStorage<'a, world::components::Direction>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (p, d, _) in (&data.physics, &mut data.direction, &data.tie).join() {
            p.handle().map(|h| {
                // TODO: Maybe use try_borrow to avoid panics (but maybe it isn't necessary here)

                let quat = h.borrow().position().rotation.quaternion().clone();
                d.0 = convert::to_cgmath_quaternion(quat);
            });
        }
    }
}
