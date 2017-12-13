//! A system to update the delta time

use specs;
use time::Duration;

use std::time::Instant;

#[derive(Clone, Copy, Debug)]
pub struct Delta(Duration);

impl Delta {
    pub fn to_float(&self) -> ::Float {
        // One billion divided by the number of nanoseconds
        self.0.num_nanoseconds().unwrap_or_else(|| {
            warn!("Delta time overflow");
            1_000_000_000
        }) as ::Float / 1_000_000_000.0
    }
}

impl Default for Delta {
    fn default() -> Self {
        Delta(Duration::milliseconds(16))
    }
}

/// A system to update the delta time
pub(crate) struct System {
    last_update: Instant,
}

impl System {
    pub fn new() -> Self {
        Self { last_update: Instant::now() }
    }
}

impl<'a> specs::System<'a> for System {
    type SystemData = specs::FetchMut<'a, Delta>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.0 = Duration::from_std(self.last_update.elapsed()).expect(
            "Delta duration conversion failure",
        );
        self.last_update = Instant::now();
    }
}
