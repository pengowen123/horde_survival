//! A custom force generator that acts as a spring used to make the player's physics body float

use common::nphysics3d::math::Force;
use common::na;

// TODO: Fine tune these
const SPRING_LENGTH_CUTOFF: ::Float = 8.0;
const SPRING_VELOCITY_CUTOFF: ::Float = 4.0;

pub struct Spring {
    pub current_length: Option<::Float>,
    pub current_velocity: ::Float,
    length: ::Float,
    stiffness: ::Float,
    friction: ::Float,
}

impl Spring {
    pub fn new(
        length: ::Float,
        stiffness: ::Float,
        friction: ::Float,
    ) -> Self {
        Self {
            current_length: None,
            current_velocity: 0.0,
            length,
            stiffness,
            friction,
        }
    }

    /// Returns the resting length of this `Spring`
    pub fn length(&self) -> ::Float {
        self.length
    }

    /// Sets the current length of this `Spring`
    pub fn set_current_length(&mut self, length: ::Float) {
        self.current_length = Some(length);
    }

    pub fn reset_current_length(&mut self) {
        self.current_length = None;
    }
}

impl Spring {
    pub fn apply(&mut self) -> Option<Force<::Float>> {
        let current_length = if let Some(l) = self.current_length {
            l
        } else {
            return None;
        };

        let delta_length = self.length - current_length;

        if (delta_length.abs() > SPRING_LENGTH_CUTOFF) ||
            self.current_velocity > SPRING_VELOCITY_CUTOFF
        {
            return None;
        }

        let total_force = {
            let force = Force::linear(na::Vector3::z() * delta_length * self.stiffness);
            let dampener = Force::linear(na::Vector3::z() * -self.current_velocity * self.friction);

            force + dampener
        };

        println!("length: {}", current_length);

        Some(total_force)
    }
}
