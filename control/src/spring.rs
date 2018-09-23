//! A custom force generator that acts as a spring used to make the player's physics body float

use common::na;
use common::nphysics3d::math::Force;

// TODO: Fine tune these
pub const SPRING_LENGTH_DELTA_DISABLE_PERCENTAGE: ::Float = 0.4;
const SPRING_LENGTH_RE_ENABLE_PERCENTAGE: ::Float = 0.9;

pub struct Spring {
    pub current_length: Option<::Float>,
    pub current_velocity: ::Float,
    length: ::Float,
    stiffness: ::Float,
    friction: ::Float,
    /// The first field is whether the spring is enabled.
    /// The second field is whether to disable re-enabling until the re-enable length has been
    /// surpassed at least once.
    enabled: (bool, bool),
}

impl Spring {
    pub fn new(length: ::Float, stiffness: ::Float, friction: ::Float) -> Self {
        Self {
            current_length: None,
            current_velocity: 0.0,
            length,
            stiffness,
            friction,
            enabled: (true, false),
        }
    }

    /// Returns the resting length of this `Spring`
    pub fn length(&self) -> ::Float {
        self.length
    }

    /// Sets the current length of this `Spring`
    ///
    /// Disables this `Spring` if the length changes by more than
    /// `SPRING_LENGTH_DELTA_DISABLE_PERCENTAGE * self.length`
    pub fn set_current_length(&mut self, length: ::Float) {
        if let Some(current_length) = self.current_length {
            let delta = (length - current_length).abs();

            if delta > SPRING_LENGTH_DELTA_DISABLE_PERCENTAGE * self.length {
                self.enabled = (false, false);
            }
        }

        self.current_length = Some(length);
    }

    /// Enables this `Spring`
    pub fn enable(&mut self) {
        self.enabled = (true, false);
    }

    /// Resets the flag that `Spring::disable_until_reenable_threshold` sets
    pub fn reset_wait_for_threshold_flag(&mut self) {
        self.enabled.1 = false;
    }

    /// Resets the current length of this `Spring`
    pub fn reset_current_length(&mut self) {
        self.current_length = None;
    }

    /// Disables this `Spring`
    ///
    /// It will automatically be re-enabled once the spring length falls below a certain value.
    /// However, it will not be re-enabled until the spring length has been above that value at
    /// least once.
    pub fn disable_until_reenable_threshold(&mut self) {
        self.enabled = (false, true);
    }

    /// Returns whether this `Spring` is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.0
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

        // If the length surpasses the length threshold, disable that check (effectively flags that
        // the threshold was reached)
        if self.enabled.1 {
            if current_length >= self.length * SPRING_LENGTH_RE_ENABLE_PERCENTAGE {
                self.enabled.1 = false;
            }
        }

        if !self.enabled.1 && current_length < self.length * SPRING_LENGTH_RE_ENABLE_PERCENTAGE {
            self.enable();
        }

        if !self.is_enabled() {
            return None;
        }

        let total_force = {
            let force = Force::linear(na::Vector3::z() * delta_length * self.stiffness);
            let dampener = Force::linear(na::Vector3::z() * -self.current_velocity * self.friction);

            force + dampener
        };

        Some(total_force)
    }
}
