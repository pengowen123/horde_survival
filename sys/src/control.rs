//! A component and system to allow other systems to control entities
//!
//! For example, the player control system may write to this component to cause the player entity to
//! move forward

// TODO: Use this module to control player physics body rotation (but only control yaw, ignore
//       pitch)

use specs::{self, DispatcherBuilder, Join};
use cgmath::{self, Quaternion};
use na;

use physics::components;
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
    /// Set the velocity to moving at the provided speed, in the direction the entity is facing
    /// If the second field is set to `true`, only take into account the entity's yaw
    MoveForward(::Float, bool),
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

    /// Sets the velocity of the entity so that it moves at the provided speed in the direction it
    /// is facing
    pub fn move_forward(&mut self, speed: ::Float) {
        self.velocity = Some(VelocityModifier::MoveForward(speed, false));
    }

    /// Like `Control::move_forward`, but only takes into account the yaw of the entity
    ///
    /// This should be used for walking entities, while `Control::move_forward` should be used for
    /// flying.
    pub fn move_forward_heading(&mut self, speed: ::Float) {
        self.velocity = Some(VelocityModifier::MoveForward(speed, true));
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
    physics: specs::WriteStorage<'a, components::Physics>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    // TODO: Finish a basic implementation of this system, then move on to better graphics
    //       See player/control.rs for how to get the forward vector (for other vectors, look it
    //       up)
    fn run(&mut self, mut data: Self::SystemData) {
        for (c, p) in (&mut data.control, &mut data.physics).join() {
            if let Some(direction) = c.direction {
                let na_quat = convert::to_na_quaternion(direction);
                p.handle().map(|h| h.borrow_mut().set_rotation(na_quat));

                c.direction = None;
            }

            if let Some(modifier) = c.velocity {
                p.handle().map(|h| {
                    let vel = h.borrow().lin_vel();
                    // FIXME: this doesn't work
                    h.borrow_mut().set_lin_vel([0.0, 0.0, vel[2]].into());

                    match modifier {
                        VelocityModifier::SetTo(velocity) => {
                            let velocity = convert::to_na_vector(velocity);
                            h.borrow_mut().set_lin_vel(velocity);
                        }
                        VelocityModifier::MoveForward(speed, yaw_only) => {
                            let mut body = h.borrow_mut();

                            let velocity = (body.position().rotation * na::Vector3::z()) * speed;
                            body.set_lin_vel(velocity);
                        }
                    }
                });
                c.velocity = None;
            }
        }
    }
}

/// Initialization of control-related systems and components
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {

    // Register components
    world.register::<Control>();

    // Add systems
    dispatcher.add(System, "control", &[])
}
