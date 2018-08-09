//! A component and system to allow other systems to control entities
//!
//! For example, the player control system may write to this component to cause the player entity to
//! move forward

extern crate common;
#[macro_use]
extern crate shred_derive;
extern crate math;

use common::specs::{self, DispatcherBuilder, Join};
use common::cgmath::{self, Quaternion};
use common::{Float, shred, na, physics};
use common::nphysics3d::world::World;
use common::nphysics3d::object::BodyMut;
use math::convert;

/// Controlled properties of an entity
pub struct Control {
    direction: Option<Quaternion<::Float>>,
    velocity: Option<VelocityModifier>,
}

/// A modifier to be applied to the velocity of an entity
#[derive(Clone, Copy, Debug)]
pub enum VelocityModifier {
    /// Set the velocity to the value
    SetTo(cgmath::Vector3<::Float>),
    /// Set the velocity to moving at the provided speed, in the provided direction
    MoveForward(Quaternion<::Float>, ::Float),
}

impl Control {
    pub fn new(direction: Option<Quaternion<::Float>>, velocity: Option<VelocityModifier>) -> Self {
        Self {
            direction,
            velocity,
        }
    }

    /// Sets the direction of the entity to the provided quaternion
    pub fn set_rotation(&mut self, direction: Quaternion<::Float>) {
        self.direction = Some(direction);
    }

    /// Sets the velocity of the entity to the provided value
    ///
    /// The velocity is reset every update, so this must be called every update in order for the
    /// velocity to persist.
    pub fn set_velocity(&mut self, velocity: cgmath::Vector3<::Float>) {
        self.velocity = Some(VelocityModifier::SetTo(velocity));
    }

    /// Sets the velocity of the entity so that it moves at the provided speed in the provided
    /// direction
    pub fn move_in_direction(&mut self, direction: Quaternion<::Float>, speed: ::Float) {
        self.velocity = Some(VelocityModifier::MoveForward(direction, speed));
    }
}

impl Default for Control {
    fn default() -> Self {
        Control::new(None, None)
    }
}

impl specs::Component for Control {
    type Storage = specs::VecStorage<Self>;
}

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    control: specs::WriteStorage<'a, Control>,
    physics: specs::WriteStorage<'a, physics::Physics>,
    world: specs::WriteExpect<'a, World<::Float>>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (c, p) in (&mut data.control, &mut data.physics).join() {
            // FIXME: Implement this
            if let Some(direction) = c.direction {
                c.direction = None;
            }

            let new_velocity = c.velocity.map(|modifier| {
                match modifier {
                    VelocityModifier::SetTo(velocity) => {
                        convert::to_na_vector(velocity)
                    }
                    VelocityModifier::MoveForward(direction, speed) => {
                        let direction = convert::to_na_quaternion(direction);
                        (direction * -na::Vector3::z()).normalize() * speed
                    }
                }
            });

            c.velocity = None;

            match data.world.body_mut(p.get_root_handle()) {
                // The `control` system only works for rigid bodies
                BodyMut::RigidBody(body) => {
                    if let Some(vel) = new_velocity {
                        body.set_linear_velocity(vel);

                        // The body must be activated because if it is sleeping then setting the velocity
                        // won't do anything
                        body.activate();
                    } else {
                        let vel_z = body.velocity().linear.z;

                        body.set_linear_velocity(na::Vector3::new(0.0, 0.0, vel_z));
                    }
                },
                // TODO: Maybe use a multibody for controlled entities to allow for joints
                //       This is blocked on nphysics#127
                BodyMut::Multibody(multibody) => {
                    if new_velocity.is_some() {
                        // The body must be activated because if it is sleeping then setting the velocity
                        // won't do anything
                        multibody.activate();
                    }

                    let velocity = &mut multibody.generalized_velocity_slice_mut()[..3];

                    // Reset horizontal velocity
                    velocity[0] = 0.0;
                    velocity[1] = 0.0;

                    if let Some(vel) = new_velocity {
                        velocity[0] = vel[0];
                        velocity[1] = vel[1];
                        velocity[2] = vel[2];
                    }
                },
                _ => continue,
            };
        }
    }
}

/// Initialization of control-related systems and components
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {

    // Register components
    world.register::<Control>();

    // Add systems
    dispatcher.with(System, "control", &[])
}
