//! A custom force generator that is used to apply movements to the player's physics body

use common::nphysics3d::math::Force;
use common::cgmath::{self, Zero};
use common::na;

pub struct MovementForceGenerator {
    horizontal_velocity: cgmath::Vector2<::Float>,
    acceleration: ::Float,
    max_speed: ::Float,
    walk_dir: Option<cgmath::Vector2<::Float>>,
}

impl MovementForceGenerator {
    pub fn new(acceleration: ::Float, max_speed: ::Float) -> Self {
        Self {
            horizontal_velocity: cgmath::Vector2::zero(),
            acceleration,
            max_speed,
            walk_dir: None,
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

    /// Updates the horizontal velocity of the entity that this force generator is applying to
    pub fn update_horizontal_velocity(&mut self, new_velocity: cgmath::Vector2<::Float>) {
        self.horizontal_velocity = new_velocity;
    }

    pub fn max_speed(&self) -> ::Float {
        self.max_speed
    }
}

impl MovementForceGenerator {
    pub fn apply(&mut self) -> Option<Force<::Float>> {
        self.walk_dir.map(|dir| {
            let dir = na::Vector3::new(dir.x, dir.y, 0.0);

            Force::linear(dir * self.acceleration)
        })
    }
}
