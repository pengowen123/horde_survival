//! A force generator composed from the spring and the movement force generators

use common::nphysics3d::force_generator::{ForceGenerator};
use common::nphysics3d::math::Force;
use common::nphysics3d::solver::IntegrationParameters;
use common::nphysics3d::object::{BodyHandle, BodySet};
use common::{cgmath, na};

use spring;
use movement;

pub struct ControllerForceGenerator {
    pub spring: spring::Spring,
    pub movement: movement::MovementForceGenerator,
    body: BodyHandle,
}

impl ControllerForceGenerator {
    pub fn new(
        spring: spring::Spring,
        movement: movement::MovementForceGenerator,
        body: BodyHandle,
    ) -> Self {
        Self {
            spring,
            movement,
            body,
        }
    }

    pub fn update_current_entity_velocity(&mut self, new_velocity: na::Vector3<::Float>) {
        self.spring.current_velocity = new_velocity[2];
        self.movement.update_horizontal_velocity(
            cgmath::Vector2::new(new_velocity[0], new_velocity[1])
        );
    }
}

impl ForceGenerator<::Float> for ControllerForceGenerator {
    fn apply(
        &mut self,
        _params: &IntegrationParameters<::Float>,
        bodies: &mut BodySet<::Float>,
    ) -> bool {
        if !bodies.contains(self.body) {
            return false;
        }

        let mut total_force = Force::zero();
        let mut applied = false;

        if let Some(force) = self.spring.apply() {
            total_force += force;
            applied = true;
        }

        if let Some(force) = self.movement.apply() {
            total_force += force;
            applied = true;
        }
        
        if applied {
            bodies.body_part_mut(self.body).apply_force(&total_force);
        }

        applied
    }
}
