//! Various flags for each entity

/// Whether an entity should be affected by gravity
#[derive(Clone, PartialEq, Eq)]
pub enum HasGravity {
    True,
    False,
}

/// Whether an entity should be controlled by the AI
#[derive(Clone, PartialEq, Eq)]
pub enum HasAI {
    True,
    False,
}

/// Whether an entity is a dummy (e.g. an attack projectile)
#[derive(Clone, PartialEq, Eq)]
pub enum IsDummy {
    True,
    False,
}

/// Which team the entity is on
#[derive(Clone, PartialEq, Eq)]
pub enum Team {
    Players,
    Monsters,
}
