//! Components related to the player
//! These components must only exist on the player (failure to uphold this invariant may cause
//! unexpected behavior)

use specs::{self, DispatcherBuilder};

use math::direction;

/// A flag used to select the player entities in systems
#[derive(Clone, Copy, Debug, Default)]
pub struct Player;

/// A camera used for rendering
#[derive(Clone, Debug, Default)]
pub struct Camera {
    direction: direction::Direction,
}

impl specs::Component for Player {
    type Storage = specs::NullStorage<Self>;
}

impl specs::Component for Camera {
    type Storage = specs::VecStorage<Self>;
}

/// Initializes player-related components
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {

    world.register::<Player>();
    world.register::<Camera>();

    dispatcher
}
