//! A custom force generator that is used to apply movements to the player's physics body

use common::nphysics3d::math::Force;
use common::cgmath::{self, InnerSpace, Zero};
use common::na;

const GROUND_STEEPNESS_FORCE: ::Float = 5000.0;

pub struct MovementForceGenerator {
    horizontal_velocity: cgmath::Vector2<::Float>,
    acceleration: ::Float,
    max_speed: ::Float,
    jump_strength: ::Float,
    walk_dir: Option<cgmath::Vector2<::Float>>,
    ground_normal: cgmath::Vector3<::Float>,
}

impl MovementForceGenerator {
    pub fn new(acceleration: ::Float, max_speed: ::Float, jump_strength: ::Float) -> Self {
        Self {
            horizontal_velocity: cgmath::Vector2::zero(),
            acceleration,
            max_speed,
            jump_strength,
            walk_dir: None,
            ground_normal: cgmath::Vector3::unit_z(),
        }
    }

    /// Apply forces to walk in the provided direction
    pub fn set_walk_direction(&mut self, dir: cgmath::Vector2<::Float>) {
        self.walk_dir = Some(dir);
    }

    /// Stop applying walking forces
    pub fn reset_walk_direction(&mut self) {
        self.walk_dir = None;
    }

    /// Updates the normal vector of the ground directly beneath the entity that this force
    /// generator is applying to

    /// Returns whether the ground steepness force will be applied (if the ground is too steep to
    /// stand on)
    pub fn update_ground_normal(&mut self, new_normal: cgmath::Vector3<::Float>) -> bool {
        self.ground_normal = new_normal;
        self.get_ground_steepness_force().is_some()
    }

    /// If the ground beneath the entity that this force generator is applying to is too steep to
    /// stand on, returns `Some` with the force that should be applied to cause the entity to slide
    /// down the hill
    pub fn get_ground_steepness_force(&self) -> Option<Force<::Float>> {
        let ground_angle = self.ground_normal.angle(cgmath::Vector3::unit_z());

        if ground_angle > cgmath::Rad(3.141 / 4.0) {
            // Apply the force in the direction of the normal, ignoring the vertical component
            let force_dir = na::Vector3::new(self.ground_normal.x, self.ground_normal.y, 0.0)
                .normalize();

            Some(Force::linear(force_dir * ground_angle.0 * GROUND_STEEPNESS_FORCE))
        } else {
            None
        }
    }

    /// Updates the horizontal velocity of the entity that this force generator is applying to
    pub fn update_horizontal_velocity(&mut self, new_velocity: cgmath::Vector2<::Float>) {
        self.horizontal_velocity = new_velocity;
    }

    /// Returns the maximum speed of this `MovementForceGenerator`
    pub fn max_speed(&self) -> ::Float {
        self.max_speed
    }

    /// Returns the jump strength of this `MovementForceGenerator`
    pub fn jump_strength(&self) -> ::Float {
        self.jump_strength
    }
}

impl MovementForceGenerator {
    pub fn apply(&mut self) -> Option<Force<::Float>> {
        let ground_steepness_force = self.get_ground_steepness_force();

        let walk_force = self.walk_dir.map(|dir| {
            let dir = na::Vector3::new(dir.x, dir.y, 0.0);

            Force::linear(dir * self.acceleration)
        });

        let mut force = None;

        if let Some(ground_steepness_force) = ground_steepness_force {
            return Some(ground_steepness_force);
        }

        if let Some(walk_force) = walk_force {
            force = force
                .map(|f| f + walk_force)
                .or_else(|| Some(walk_force));
        }

        force
    }
}
