//! A component and system to allow other systems to control entities
//!
//! For example, the player control system may write to this component to cause the player entity to
//! move forward

extern crate common;
#[macro_use]
extern crate shred_derive;
extern crate math;

mod spring;
mod movement;
mod controller;

pub use self::spring::Spring;
pub use self::movement::MovementForceGenerator;

use common::specs::{self, DispatcherBuilder, Join};
use common::cgmath::{self, Quaternion};
use common::{Float, shred, na, physics};
use common::nphysics3d::world::World;
use common::nphysics3d::object::{Body, BodyMut, BodyHandle, ColliderHandle};
use common::nphysics3d::force_generator::ForceGeneratorHandle;
use common::ncollide3d::query::{self, RayCast};
use common::cgmath::InnerSpace;
use math::convert;

/// Controlled properties of an entity
pub struct Control {
    force_generator: ForceGeneratorHandle,
    direction: Option<Quaternion<::Float>>,
    velocity: Option<VelocityModifier>,
    jump: bool,
    friction: ::Float,
    max_speed: ::Float,
}

/// A modifier to be applied to the velocity of an entity
#[derive(Clone, Copy, Debug)]
pub enum VelocityModifier {
    /// Walk horizontally in the provided direction, ignoring the vertical component
    WalkForward(cgmath::Vector2<::Float>),
}

impl Control {
    /// Returns a new `Control`
    pub fn new(
        body_handle: BodyHandle,
        movement: movement::MovementForceGenerator,
        spring: spring::Spring,
        friction: ::Float,
        world: &mut World<::Float>,
    ) -> Self {
        let max_speed = movement.max_speed();
        let force_generator = world.add_force_generator(
            controller::ControllerForceGenerator::new(
                spring,
                movement,
                body_handle,
            )
        );

        Self {
            force_generator,
            friction,
            direction: None,
            velocity: None,
            jump: false,
            max_speed,
        }
    }

    /// Sets the direction of the entity to the provided quaternion
    pub fn set_rotation(&mut self, direction: Quaternion<::Float>) {
        self.direction = Some(direction);
    }

    /// Makes the entity walk horizontally in the provided direction, ignoring the vertical
    /// component
    pub fn walk_in_direction(&mut self, direction: cgmath::Vector2<::Float>) {
        self.velocity = Some(VelocityModifier::WalkForward(direction));
    }

    /// Makes the entity jump
    pub fn jump(&mut self) {
        self.jump = true;
    }
}

impl specs::Component for Control {
    type Storage = specs::VecStorage<Self>;
}

pub struct FloorColliderHandle(Option<ColliderHandle>);

impl FloorColliderHandle {
    pub fn set_handle(&mut self, handle: ColliderHandle) {
        self.0 = Some(handle);
    }

    pub fn get_handle(&self) -> Option<ColliderHandle> {
        self.0
    }
}

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    control: specs::WriteStorage<'a, Control>,
    physics: specs::WriteStorage<'a, physics::Physics>,
    world: specs::WriteExpect<'a, World<::Float>>,
    floor_handle: specs::ReadExpect<'a, FloorColliderHandle>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (c, p) in (&mut data.control, &mut data.physics).join() {
            // FIXME: Implement this
            if let Some(direction) = c.direction {
                c.direction = None;
            }

            let walk_dir = c.velocity.map(|direction| {
                match direction {
                    VelocityModifier::WalkForward(direction) => direction.normalize()
                }
            });

            match data.world.body_mut(p.get_root_handle()) {
                // The `control` system only works for rigid bodies
                // TODO: Maybe use a multibody for controlled entities to allow for joints
                //       This is blocked on nphysics#127
                BodyMut::RigidBody(body) => {
                    body.set_angular_velocity(na::zero());

                    // Only apply friction is the entity is not trying to walk
                    if c.velocity.is_none() {
                        let mut vel = body.velocity().linear;

                        // Apply friction to the entity's horizontal velocity
                        vel[0] *= c.friction;
                        vel[1] *= c.friction;

                        body.set_linear_velocity(vel);
                    } else {
                        let mut vel = body.velocity().linear;

                        let magnitude = cgmath::Vector2::new(vel[0], vel[1]).magnitude();

                        if magnitude > c.max_speed {
                            vel[0] = vel[0] / magnitude * c.max_speed;
                            vel[1] = vel[1] / magnitude * c.max_speed;
                        }

                        body.set_linear_velocity(vel);

                        // If the entity is trying to walk, activate its physics body and apply the
                        // movement speed limit
                        body.activate();
                    }
                },
                _ => continue,
            }

            c.velocity = None;

            let mut current_spring_length = None;
            let mut ground_normal = None;

            if let Some(handle) = p.get_root_collider() {
                if let Some(entity_collider) = data.world.collider(handle) {
                    if let Some(floor_handle) = data.floor_handle.get_handle() {
                        if let Some(floor_collider) = data.world.collider(floor_handle) {
                            let entity_pos =
                                na::Point3::origin() +
                                entity_collider.position().translation.vector;

                            let ray = query::Ray::new(entity_pos, -na::Vector3::z());

                            let intersection = floor_collider
                                .shape()
                                .toi_and_normal_with_ray(floor_collider.position(), &ray, false);

                            if let Some(intersection) = intersection {
                                current_spring_length = Some(intersection.toi);
                                ground_normal = Some(intersection.normal);
                            }
                        }
                    }
                }
            }

            let current_entity_velocity = {
                if let Body::RigidBody(rb) = data.world.body(p.get_root_handle()) {
                    rb.velocity().linear
                } else {
                    na::zero()
                }
            };

            let (is_ground_too_steep, set_vertical_velocity, spring_enabled) = {
                let mut controller = data.world
                    .force_generator_mut(c.force_generator)
                    .downcast_mut::<controller::ControllerForceGenerator>().unwrap();

                if let Some(length) = current_spring_length {
                    controller.spring.set_current_length(length);
                } else {
                    println!("No collision");
                    controller.spring.reset_current_length();
                }

                let is_ground_too_steep = controller.movement.update_ground_normal(
                    ground_normal
                        .map(convert::to_cgmath_vector)
                        .unwrap_or(cgmath::Vector3::unit_z())
                );

                if is_ground_too_steep {
                    controller.spring.reset_wait_for_threshold_flag();
                }

                if let Some(walk_dir) = walk_dir {
                    controller.movement.set_walk_direction(walk_dir);
                } else {
                    controller.movement.reset_walk_direction();
                }

                let jump_velocity = if c.jump {
                    c.jump = false;
                    if controller.spring.is_enabled() {
                        controller.spring.disable_until_reenable_threshold();
                        Some(controller.movement.jump_strength())
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Update the velocity fields on the controller's force generators
                controller.update_current_entity_velocity(current_entity_velocity);

                (is_ground_too_steep, jump_velocity, controller.spring.is_enabled())
            };

            if let BodyMut::RigidBody(rb) = data.world.body_mut(p.get_root_handle()) {
                if is_ground_too_steep {
                    let mut vel = *rb.velocity();

                    vel.linear[0] = 0.0;
                    vel.linear[1] = 0.0;

                    // If the entity is jumping, don't reset its vertical velocity because it would
                    // cause strange behavior when an entity jumps against a wall that is too steep
                    // to stand on
                    if spring_enabled {
                        vel.linear[2] = 0.0;
                    }

                    rb.set_velocity(vel);
                }

                if let Some(new_vel) = set_vertical_velocity {
                    // Entities cannot jump if the ground beneath them is too steep to stand on
                    if !is_ground_too_steep {
                        let mut vel = *rb.velocity();

                        vel.linear[2] = new_vel;

                        rb.set_velocity(vel);
                    }
                }
            }
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

    // Add resources
    world.add_resource(FloorColliderHandle(None));

    // Add systems
    dispatcher.with(System, "control", &[])
}
