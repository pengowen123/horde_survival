//! Components related to the player entity
//!
//! These components must only exist on one entity (failure to uphold this may cause unexpected
//! behavior)

use specs;

/// A flag that represents an entity being the player entity
///
/// The entity with this component will be controlled by the player
#[derive(Clone, Copy, Debug, Default)]
pub struct Player;

impl specs::Component for Player {
    type Storage = specs::NullStorage<Self>;
}
