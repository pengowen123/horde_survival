//! A component and system to allow other systems to control entities
//!
//! For example, the player control system may write to this component to cause the player entity to
//! move forward

// TODO: Use this module to control player physics body rotation (but only control yaw, ignore
//       pitch)

use specs::{self, DispatcherBuilder, Join};
use cgmath::{self, Quaternion};

use physics::components;
use math::convert;

/// Controlled properties of an entity
pub struct Control {
    direction: Option<cgmath::Quaternion<::Float>>,
}

impl Control {
    pub fn new(direction: Option<Quaternion<::Float>>) -> Self {
        Self { direction }
    }
}

impl Default for Control {
    fn default() -> Self {
        Control::new(None)
    }
}

impl specs::Component for Control {
    type Storage = specs::VecStorage<Self>;
}

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    control: specs::ReadStorage<'a, Control>,
    physics: specs::WriteStorage<'a, components::Physics>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (c, p) in (&data.control, &mut data.physics).join() {
            if let Some(direction) = c.direction {
                let direction = convert::to_na_quaternion(direction);
                p.handle().map(|h| h.borrow_mut().set_rotation(direction));
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
