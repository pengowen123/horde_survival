//! A component and system to tie the position of an entity to the position of its physics body

use specs::{self, Join};
use common;

use physics;
use math::convert;

/// This component acts as a flag to enable the overwriting of an entity's position with the
/// position of its physics body
pub struct PhysicsTiedPosition;

impl specs::Component for PhysicsTiedPosition {
    type Storage = specs::VecStorage<Self>;
}

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    tie: specs::ReadStorage<'a, PhysicsTiedPosition>,
    physics: specs::ReadStorage<'a, physics::components::Physics>,
    space: specs::WriteStorage<'a, common::Position>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (p, s, _) in (&data.physics, &mut data.space, &data.tie).join() {
            p.handle().map(|h| {
                // TODO: Maybe use try_borrow to avoid panics (but maybe it isn't necessary here)
                let pos = h.borrow().position_center();
                s.0 = convert::to_cgmath_point(pos);
            });
        }
    }
}
