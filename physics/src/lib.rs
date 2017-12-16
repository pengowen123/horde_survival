//! Components and systems for physics simulation
//!
//! The physics system internally uses the `nphysics3d` physics engine, so it exists as a wrapper
//! with components to provide input to the engine, and to read from the results.

extern crate common;
extern crate math;
#[macro_use]
extern crate shred_derive;

mod init;
mod output;
mod scale;

pub use init::initialize;

#[allow(unused_imports)]
use common::shred;
use common::{Float, Scale, Delta, na, ncollide, nphysics3d};
use common::specs::{self, Join};
use common::nphysics3d::world::World;
use common::nphysics3d::object::{RigidBody, RigidBodyHandle};
use common::physics;

pub struct System {
    world: World<::Float>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    physics: specs::WriteStorage<'a, physics::Physics>,
    delta: specs::Fetch<'a, Delta>,
    entities: specs::Entities<'a>,
    scale: specs::WriteStorage<'a, Scale>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let delta = data.delta.to_float();

        for (entity, p) in (&*data.entities, &mut data.physics).join() {
            // Initialize new entities and add them to the world
            let mut new_handle = None;
            if let physics::Handle::Init(ref mut init) = *p.handle_mut() {
                let mut init = init.take().expect(
                    "Attempt to initialize physics body multiple times",
                );

                new_handle = Some(self.add_entity(&mut *init));
            }

            if let Some(h) = new_handle {
                p.set_handle(h);
            }

            // Apply changes to the scale of entities
            if let Some(scale) = data.scale.get_mut(entity) {
                if let Some(old) = scale.get_previous_value() {
                    let relative_scale = scale.get() / old;

                    let new_handle = {
                        let rb = p.handle_mut().get_body_mut().expect(
                            "Found uninitialized handle",
                        );

                        let new_rb = scale::scale_body(&*rb, relative_scale as ::Float);

                        self.replace_rigid_body(rb, new_rb)
                    };

                    p.set_handle(new_handle);

                    scale.reset_flag();
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

            if let physics::Handle::Body(ref h) = *handle {
                if p.lock_rotation() {
                    let mut h = h.borrow_mut();

                    h.set_ang_vel(na::Vector3::new(0.0, 0.0, 0.0));
                    h.set_rotation(na::UnitQuaternion::from_quaternion(
                        na::Quaternion::new(1.0, 0.0, 0.0, 0.0),
                    ));
                }
            }
        }
    }
}

impl System {
    /// Adds the rigid body provided by the provided initialization function to the physics world,
    /// and returns a handle to it
    fn add_entity<F>(&mut self, mut init: F) -> RigidBodyHandle<::Float>
    where
        F: FnMut() -> RigidBody<::Float>,
    {
        let body = init();
        self.world.add_rigid_body(body)
    }

    /// Removes the `old` body with the `new` body in the physics world, and returns a handle to
    /// the new one
    fn replace_rigid_body(
        &mut self,
        old: &RigidBodyHandle<::Float>,
        new: RigidBody<::Float>,
    ) -> RigidBodyHandle<::Float> {
        self.world.remove_rigid_body(old);
        self.world.add_rigid_body(new)
    }

    /// Removes physics bodies belonging to entities that were removed
    // TODO: Implement this
    fn remove_dead_bodies(&mut self) {}
}
