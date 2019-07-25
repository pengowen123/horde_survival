//! A force generator composed from the spring and the movement force generators

use common::nphysics3d::force_generator::ForceGenerator;
use common::nphysics3d::math::ForceType;
use common::nphysics3d::object::{BodyPartHandle, BodySet};
use common::nphysics3d::solver::IntegrationParameters;
use common::{cgmath, na};

use movement;
use spring;

/// The result of calling the `apply` function of a force generator
pub enum ForceGeneratorResult {
    None,
    BodyRemoved,
    Some(na::Vector3<::Float>),
}

pub struct ControllerForceGenerator {
    pub spring: spring::Spring,
    pub movement: movement::MovementForceGenerator,
    body: BodyPartHandle,
}

impl ControllerForceGenerator {
    pub fn new(
        spring: spring::Spring,
        movement: movement::MovementForceGenerator,
        body: BodyPartHandle,
    ) -> Self {
        Self {
            spring,
            movement,
            body,
        }
    }

    pub fn update_current_entity_velocity(&mut self, new_velocity: na::Vector3<::Float>) {
        self.spring.current_velocity = new_velocity[2];
        self.movement
            .update_horizontal_velocity(cgmath::Vector2::new(new_velocity[0], new_velocity[1]));
    }
}

impl ForceGenerator<::Float> for ControllerForceGenerator {
    fn apply(
        &mut self,
        _params: &IntegrationParameters<::Float>,
        bodies: &mut BodySet<::Float>,
    ) -> bool {
        if !bodies.contains(self.body.0) {
            return false;
        }

        let mut total_force = na::zero();
        let mut applied = false;

        match self.spring.apply() {
            ForceGeneratorResult::None => {},
            ForceGeneratorResult::Some(force) => {
                total_force += force;
                applied = true;
            },
            ForceGeneratorResult::BodyRemoved => return false,
        }

        match self.movement.apply() {
            ForceGeneratorResult::None => {},
            ForceGeneratorResult::Some(force) => {
                total_force += force;
                applied = true;
            },
            ForceGeneratorResult::BodyRemoved => return false,
        }

        if applied {
            if let Some(ref mut body) = bodies.body_mut(self.body.0) {
                // TODO: Maybe customize some of these parameters to improve the player controller
                body.apply_force_at_local_point(
                    self.body.1,
                    &total_force,
                    &na::Point3::origin(),
                    ForceType::Force,
                    false,
                );
            }
        }

        true
    }
}
