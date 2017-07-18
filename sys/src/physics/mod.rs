//! Components and systems for physics simulation
//!
//! The physics system internally uses the `nphysics3d` physics engine, so it exists as a wrapper
//! with components to provide input to the engine, and to read from the results.

pub mod components;
pub mod init;
mod handle;
mod output;

use specs::{self, Join};
use nphysics3d::world::World;
use na;

use physics;
use delta;

pub struct System {
    world: World<::Float>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    physics: specs::WriteStorage<'a, physics::components::Physics>,
    delta: specs::Fetch<'a, delta::Delta>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let delta = data.delta.to_float();

        for p in (&mut data.physics).join() {
            match *p.handle() {
                handle::Handle::Body(_) => {}
                handle::Handle::Init(f) => {
                    let body = f();
                    let handle = self.world.add_rigid_body(body);
                    p.set_handle(handle);
                }
            }
        }

        // Simulate the world for `delta` seconds
        self.world.step(delta);

        // TODO: Finish physics system:
        //       Implement remove_dead_bodies and any other physics options like lock_rotation that
        //       may be useful

        self.remove_dead_bodies();

        for p in (&mut data.physics).join() {
            let handle = p.handle();

            if let handle::Handle::Body(ref h) = *handle {
                if p.lock_rotation() {
                    let mut h = h.borrow_mut();

                    h.set_ang_vel(na::Vector3::new(0.0, 0.0, 0.0));
                    h.set_rotation(na::UnitQuaternionBase::from_quaternion(
                        na::QuaternionBase::new(1.0, 0.0, 0.0, 0.0),
                    ));
                }
            }
        }
    }
}

impl System {
    /// Removes physics bodies belonging to entities that were removed
    // TODO: Implement this
    fn remove_dead_bodies(&mut self) {}
}
